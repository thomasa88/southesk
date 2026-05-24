// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in the system keyring.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, CredentialStore, StoredCredentials};

use crate::cred_store::TmrCredStore;

#[derive(Clone)]
pub struct KeyringCredStore {
    store: Arc<dyn keyring_core::api::CredentialStoreApi + Send + Sync>,
    user: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CombinedStoredCreds {
    #[serde(flatten)]
    rmcp_creds: Option<StoredCredentials>,
    client_secret: Option<String>,
}

impl KeyringCredStore {
    /// Creates a new keyring credential store.
    ///
    /// The `user` parameter can be used to differentiate if the current
    /// computer user needs to store credentials for multiple Montrose accounts
    /// or sessions (e.g. for testing).
    pub fn new(user: impl Into<String>) -> Result<Self, AuthError> {
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
            user: user.into(),
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
        let keyring_secret = match entry.get_secret() {
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
        let creds: CombinedStoredCreds = serde_json::from_slice(&keyring_secret).map_err(|e| {
            AuthError::InternalError(format!("Failed to deserialize credentials from JSON: {e}"))
        })?;
        Ok(Some(creds))
    }

    fn save_creds(&self, creds: CombinedStoredCreds) -> Result<(), AuthError> {
        let entry = self.get_entry()?;
        let keyring_secret = serde_json::to_vec(&creds).map_err(|e| {
            AuthError::InternalError(format!("Failed to serialize credentials to JSON: {e}"))
        })?;
        use keyring_core::error::Error;
        match entry.set_secret(&keyring_secret) {
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
        self.save_creds(creds)
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

impl TmrCredStore for KeyringCredStore {
    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        let client_secret = self.load_creds()?.and_then(|c| c.client_secret);
        debug!("Loaded client secret from keyring");
        Ok(client_secret)
    }

    async fn save_client_secret(&self, secret: impl Into<String>) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.client_secret = Some(secret.into());
        dbg!(&creds);
        self.save_creds(creds)?;
        debug!("Saved client secret to keyring");
        Ok(())
    }
}
