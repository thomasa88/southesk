// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in the system keyring.

use std::sync::Arc;
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, CredentialStore, StoredCredentials};

use super::{CombinedStoredCreds, TmrCredStore};

#[derive(Clone)]
pub struct KeyringCredStore {
    store: Arc<dyn keyring_core::api::CredentialStoreApi + Send + Sync>,
    user: String,
}

impl KeyringCredStore {
    /// Creates a new keyring credential store.
    pub fn new() -> Result<Self, AuthError> {
        #[cfg(target_os = "linux")]
        let store = dbus_secret_service_keyring_store::Store::new();
        #[cfg(target_os = "windows")]
        let store = windows_native_keyring_store::Store::new();
        #[cfg(target_os = "macos")]
        let store = apple_native_keyring_store::Store::new();
        Ok(Self {
            store: store.map_err(|e| {
                AuthError::InternalError(format!("Failed to initialize keyring store: {e}"))
            })?,
            user: "user_not_set".to_string(),
        })
    }

    fn get_entry(&self) -> Result<keyring_core::Entry, AuthError> {
        self.store
            .build("tmr-client", &self.user, None)
            .map_err(|e| {
                AuthError::InternalError(format!("Failed to build keyring entry specifier: {e}"))
            })
    }

    fn load_creds(&self) -> Result<Option<CombinedStoredCreds>, AuthError> {
        let entry = self.get_entry()?;
        let json = match entry.get_secret() {
            Ok(secret) => secret,
            Err(keyring_core::error::Error::NoEntry) => return Ok(None),
            Err(keyring_core::error::Error::Ambiguous(_)) => {
                return Err(AuthError::InternalError(
                    "Multiple matching entries in keyring when loading. Support not implemented."
                        .to_string(),
                ));
            }
            Err(e) => {
                return Err(AuthError::InternalError(format!(
                    "Unhandled keyring error when loading: {e}"
                )));
            }
        };
        super::decode_json_creds(json.as_slice())
    }

    fn save_creds(&self, creds: CombinedStoredCreds) -> Result<(), AuthError> {
        let entry = self.get_entry()?;
        let json = super::encode_json_creds(&creds)?;
        use keyring_core::error::Error;
        match entry.set_secret(&json) {
            Ok(_) => Ok(()),
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError(
                "Multiple matching entries in keyring when saving. Support not implemented."
                    .to_string(),
            )),
            Err(e) => Err(AuthError::InternalError(format!(
                "Unhandled keyring error when saving: {e}"
            ))),
        }
    }
}

#[async_trait]
impl CredentialStore for KeyringCredStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        let creds = self.load_creds()?.and_then(|c| c.rmcp_creds);
        if creds.is_some() {
            debug!("Loaded credentials from keyring");
        }
        Ok(creds)
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.rmcp_creds = Some(credentials);
        self.save_creds(creds)?;
        debug!("Saved credentials to keyring");
        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        use keyring_core::error::Error;
        match self.get_entry()?.delete_credential() {
            Ok(_) | Err(Error::NoEntry) => {
                debug!("Cleared credentials from keyring");
                Ok(())
            }
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError(
                "Multiple matching entries in keyring when clearing. Support not implemented."
                    .to_string(),
            )),
            Err(e) => Err(AuthError::InternalError(format!(
                "Unhandled keyring error when clearing: {e}"
            ))),
        }
    }
}

#[async_trait]
impl TmrCredStore for KeyringCredStore {
    fn set_user(&mut self, user: &str) {
        self.user = user.into();
    }

    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.client_secret = Some(secret.into());
        dbg!(&creds);
        self.save_creds(creds)?;
        debug!("Saved client secret to keyring");
        Ok(())
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        let client_secret = self.load_creds()?.and_then(|c| c.client_secret);
        debug!("Loaded client secret from keyring");
        Ok(client_secret)
    }

    fn dyn_clone(&self) -> Box<dyn TmrCredStore> {
        Box::new((*self).clone())
    }
}
