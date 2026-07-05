// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Credential stores for storing OAuth credentials.
#[cfg(feature = "keyring")]
mod keyring_cred_store;
mod plaintext_cred_store;

#[cfg(feature = "keyring")]
pub use keyring_cred_store::KeyringCredStore;
pub use plaintext_cred_store::PlaintextCredStore;

use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use rmcp::transport::{AuthError, CredentialStore, StoredCredentials};
use serde::{Deserialize, Serialize};

/// An extension of [`CredentialStore`] that allows saving and loading the
/// OAuth client secret.
///
/// Since the MCP API creates the client secret when the client is registered
/// and then requires it for token refreshes, it needs to be stored.
#[async_trait]
pub trait FullCredStore: CredentialStore + Debug {
    /// Saves the client secret to the credential store.
    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError>;
    /// Loads the client secret from the credential store, if it exists.
    async fn load_client_secret(&self) -> Result<Option<String>, AuthError>;
}

/// A credential store wrapper that allows the provided credential store to be
/// shared between calls and threads.
#[derive(Debug, Clone)]
pub struct SharedCredStore {
    inner: Arc<dyn FullCredStore>,
}

impl SharedCredStore {
    /// Creates a new shared credential store from the provided credential store.
    pub fn new(cred_store: impl FullCredStore + 'static) -> Self {
        Self {
            inner: Arc::new(cred_store),
        }
    }
}

#[async_trait]
impl FullCredStore for SharedCredStore {
    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        self.inner.save_client_secret(secret).await
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        self.inner.load_client_secret().await
    }
}

#[async_trait]
impl CredentialStore for SharedCredStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        self.inner.load().await
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        self.inner.save(credentials).await
    }

    async fn clear(&self) -> Result<(), AuthError> {
        self.inner.clear().await
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
        AuthError::InternalError(format!("failed to deserialize credentials from JSON: {e}"))
    })?;
    Ok(Some(creds))
}

pub(crate) fn encode_json_creds(creds: &CombinedStoredCreds) -> Result<Vec<u8>, AuthError> {
    serde_json::to_vec(creds).map_err(|e| {
        AuthError::InternalError(format!("failed to serialize credentials to JSON: {e}"))
    })
}
