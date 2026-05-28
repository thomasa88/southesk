// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

#[cfg(feature = "keyring-creds")]
pub mod keyring_cred_store;
pub mod plaintext_cred_store;

use async_trait::async_trait;
use rmcp::transport::{AuthError, CredentialStore, StoredCredentials};
use serde::{Deserialize, Serialize};

/// An extension of [`CredentialStore`] that allows saving and loading the
/// client secret.
///
/// Since the MCP API creates the client secret when the client is registered
/// and then requires it for token refreshes, it needs to be stored.
///
/// The tmr-client implementation assumes that the state is shared between all
/// credential stores initialized with the same configuration. It can be
/// implemented using a shared file, for example.
#[async_trait]
pub trait TmrCredStore: CredentialStore {
    /// Sets a user id, used to differentiate if the current computer user needs
    /// to store credentials for multiple Montrose accounts or sessions (e.g.
    /// for testing).
    fn set_user(&mut self, user: &str);

    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError>;
    async fn load_client_secret(&self) -> Result<Option<String>, AuthError>;

    fn dyn_clone(&self) -> Box<dyn TmrCredStore>;
}

#[async_trait]
impl CredentialStore for Box<dyn TmrCredStore> {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        (**self).load().await
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        (**self).save(credentials).await
    }

    async fn clear(&self) -> Result<(), AuthError> {
        (**self).clear().await
    }
}

#[async_trait]
impl TmrCredStore for Box<dyn TmrCredStore> {
    fn set_user(&mut self, user: &str) {
        (**self).set_user(user);
    }

    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        (**self).save_client_secret(secret).await
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        (**self).load_client_secret().await
    }

    fn dyn_clone(&self) -> Box<dyn TmrCredStore> {
        (**self).dyn_clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(crate) struct CombinedStoredCreds {
    #[serde(flatten)]
    rmcp_creds: Option<StoredCredentials>,
    client_secret: Option<String>,
}

pub(crate) fn decode_json_creds(json: &[u8]) -> Result<Option<CombinedStoredCreds>, AuthError> {
    let creds: CombinedStoredCreds = serde_json::from_slice(json).map_err(|e| {
        AuthError::InternalError(format!("Failed to deserialize credentials from JSON: {e}"))
    })?;
    Ok(Some(creds))
}

pub(crate) fn encode_json_creds(creds: &CombinedStoredCreds) -> Result<Vec<u8>, AuthError> {
    serde_json::to_vec(creds).map_err(|e| {
        AuthError::InternalError(format!("Failed to serialize credentials to JSON: {e}"))
    })
}
