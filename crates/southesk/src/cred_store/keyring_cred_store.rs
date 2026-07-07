// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in the user's keyring.

use std::sync::Arc;
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, CredentialStore, StoredCredentials};

use super::{CombinedStoredCreds, FullCredStore};

/// A credential store that saves the credentials in the user's keyring.
#[derive(Debug, Clone)]
pub struct KeyringCredStore {
    store: Arc<dyn keyring_core::api::CredentialStoreApi + Send + Sync>,
    service: String,
    user: String,
}

impl KeyringCredStore {
    /// Creates a new keyring credential store.
    ///
    /// The `service` parameter is used to differentiate entries for different
    /// applications. `user` can be used to differentiate different Montrose
    /// accounts.
    pub fn new(service: impl Into<String>, user: impl Into<String>) -> Result<Self, AuthError> {
        #[cfg(target_os = "linux")]
        let store = dbus_secret_service_keyring_store::Store::new();
        #[cfg(target_os = "windows")]
        let store = windows_native_keyring_store::Store::new();
        #[cfg(target_os = "macos")]
        let store = apple_native_keyring_store::keychain::Store::new();
        Ok(Self {
            store: store.map_err(|e| {
                AuthError::InternalError(format!("failed to initialize keyring store: {e}"))
            })?,
            service: service.into(),
            user: user.into(),
        })
    }

    fn get_keyring_entry(&self) -> Result<keyring_core::Entry, AuthError> {
        self.store
            .build(&self.service, &self.user, None)
            .map_err(|e| {
                AuthError::InternalError(format!("failed to build keyring entry specifier: {e}"))
            })
    }

    fn load_creds(&self) -> Result<Option<CombinedStoredCreds>, AuthError> {
        let entry = self.get_keyring_entry()?;
        let json = match entry.get_secret() {
            Ok(secret) => secret,
            Err(keyring_core::error::Error::NoEntry) => return Ok(None),
            Err(keyring_core::error::Error::Ambiguous(_)) => {
                return Err(AuthError::InternalError(
                    "multiple matching entries in keyring when loading is not supported"
                        .to_string(),
                ));
            }
            Err(e) => {
                return Err(AuthError::InternalError(format!(
                    "unhandled keyring error when loading: {e}"
                )));
            }
        };
        super::decode_json_creds(json.as_slice())
    }

    fn save_creds(&self, creds: &CombinedStoredCreds) -> Result<(), AuthError> {
        use keyring_core::error::Error;
        let entry = self.get_keyring_entry()?;
        let json = super::encode_json_creds(creds)?;
        match entry.set_secret(&json) {
            Ok(()) => Ok(()),
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError(
                "multiple matching entries in keyring when saving is not supported".to_string(),
            )),
            Err(e) => Err(AuthError::InternalError(format!(
                "unhandled keyring error when saving: {e}"
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
        self.save_creds(&creds)?;
        debug!("Saved credentials to keyring");
        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        use keyring_core::error::Error;
        match self.get_keyring_entry()?.delete_credential() {
            Ok(()) | Err(Error::NoEntry) => {
                debug!("Cleared credentials from keyring");
                Ok(())
            }
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError(
                "multiple matching entries in keyring when clearing. Support not implemented."
                    .to_string(),
            )),
            Err(e) => Err(AuthError::InternalError(format!(
                "unhandled keyring error when clearing: {e}"
            ))),
        }
    }
}

#[async_trait]
impl FullCredStore for KeyringCredStore {
    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.client_secret = Some(secret.into());
        self.save_creds(&creds)?;
        debug!("Saved client secret to keyring");
        Ok(())
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        let client_secret = self.load_creds()?.and_then(|c| c.client_secret);
        debug!("Loaded client secret from keyring");
        Ok(client_secret)
    }
}
