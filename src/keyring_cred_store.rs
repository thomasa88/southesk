// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in the system keyring.

use std::sync::Arc;
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, StoredCredentials};

pub struct KeyringCredStore {
    store: Arc<dyn keyring_core::api::CredentialStoreApi + Send + Sync>,
    user: String,
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
}

#[async_trait]
impl rmcp::transport::CredentialStore for KeyringCredStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        let entry = self.get_entry()?;
        let secret = match entry.get_secret() {
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
        let creds: StoredCredentials = serde_json::from_slice(&secret).map_err(|e| {
            AuthError::InternalError(format!("Failed to deserialize credentials from JSON: {e}"))
        })?;
        debug!("Loaded credentials from keyring");
        Ok(Some(creds))
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        let entry = self.get_entry()?;
        let secret = serde_json::to_vec(&credentials).map_err(|e| {
            AuthError::InternalError(format!("Failed to serialize credentials to JSON: {e}"))
        })?;
        use keyring_core::error::Error;
        match entry.set_secret(&secret) {
            Ok(_) => {
                debug!("Saved credentials to keyring");
                Ok(())
            }
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError(
                "Multiple matching entries in keyring when saving. Support not implemented."
                    .to_string(),
            )),
            Err(e) => Err(AuthError::InternalError(format!(
                "Unhandled keyring error when saving: {e}"
            ))),
        }
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
