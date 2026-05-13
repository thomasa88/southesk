// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in the system keyring.

use std::sync::Arc;
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, StoredCredentials};

pub struct KeyringCredStore {
    store: Arc<dyn keyring_core::api::CredentialStoreApi + Send + Sync>,
}

impl KeyringCredStore {
    pub fn new() -> Self {
        #[cfg(target_os = "linux")]
        let store = dbus_secret_service_keyring_store::Store::new();
        #[cfg(target_os = "windows")]
        let store = windows_native_keyring_store::Store::new();
        #[cfg(target_os = "macos")]
        let store = apple_native_keyring_store::Store::new();
        Self {
            store: store.unwrap(),
        }
    }

    fn get_entry(&self) -> Result<keyring_core::Entry, AuthError> {
        self.store.build("tmr-client", "", None).map_err(|err| {
            AuthError::InternalError(format!("Failed to build keyring entry specifier: {err}"))
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
                return Err(AuthError::InternalError("Multiple matching entries in keyring when loading. Support not implemented.".to_string()));
            }
            Err(e) => {
                return Err(AuthError::InternalError(format!(
                    "Unhandled keyring error when loading: {e}"
                )));
            }
        };
        let creds: StoredCredentials = serde_json::from_slice(&secret).map_err(|err| {
            AuthError::InternalError(format!(
                "Failed to deserialize credentials from JSON: {err}"
            ))
        })?;
        debug!("Loaded credentials from keyring");
        Ok(Some(creds))
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        let entry = self.get_entry()?;
        let secret = serde_json::to_vec(&credentials).map_err(|err| {
            AuthError::InternalError(format!("Failed to serialize credentials to JSON: {err}"))
        })?;
        use keyring_core::error::Error;
        match entry.set_secret(&secret) {
            Ok(_) => {
                debug!("Saved credentials to keyring");
                Ok(())
            }
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError("Multiple matching entries in keyring when saving. Support not implemented.".to_string())),
            Err(err) => Err(AuthError::InternalError(format!(
                "Unhandled keyring error when saving: {err}"
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
            Err(Error::Ambiguous(_)) => Err(AuthError::InternalError("Multiple matching entries in keyring when loading. Support not implemented.".to_string())),
            Err(err) => Err(AuthError::InternalError(format!(
                "Unhandled keyring error when loading: {err}"
            ))),
        }
    }
}
