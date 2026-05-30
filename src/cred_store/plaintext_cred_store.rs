// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in a plaintext JSON file in
//! the user's config directory.

use std::path::PathBuf;
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, StoredCredentials};

use super::{CombinedStoredCreds, TmrCredStore};

#[derive(Debug, Clone)]
pub struct PlaintextCredStore {
    path: PathBuf,
}

impl PlaintextCredStore {
    /// Creates a new plaintext credential store at the given path.
    ///
    /// Use different paths for different Montrose accounts.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    fn load_creds(&self) -> Result<Option<CombinedStoredCreds>, AuthError> {
        if !self.path.exists() {
            return Ok(None);
        }
        let file = std::fs::File::open(&self.path).map_err(|e| {
            AuthError::InternalError(format!("Failed to open credentials file: {e}"))
        })?;
        let creds: CombinedStoredCreds = serde_json::from_reader(file).map_err(|e| {
            AuthError::InternalError(format!("Failed to deserialize credentials: {e}"))
        })?;
        Ok(Some(creds))
    }

    fn save_creds(&self, creds: CombinedStoredCreds) -> Result<(), AuthError> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AuthError::InternalError(format!(
                    "Failed to create directories for credentials file {:?}: {e}",
                    self.path
                ))
            })?;
        }
        let mut options = std::fs::OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(not(target_os = "windows"))]
        {
            use std::os::unix::fs::OpenOptionsExt;
            // Only the current user can access the file
            options.mode(0o600);
        }
        let file = options.open(&self.path).map_err(|e| {
            AuthError::InternalError(format!(
                "Failed to create credentials file {:?}: {e}",
                self.path
            ))
        })?;
        serde_json::to_writer_pretty(file, &creds).map_err(|e| {
            AuthError::InternalError(format!("Failed to serialize credentials: {e}"))
        })?;
        Ok(())
    }
}

#[async_trait]
impl rmcp::transport::CredentialStore for PlaintextCredStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        let creds = self.load_creds()?.and_then(|c| c.rmcp_creds);
        if creds.is_some() {
            debug!("Loaded credentials from {:?}", self.path);
        }
        Ok(creds)
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.rmcp_creds = Some(credentials);
        self.save_creds(creds)?;
        debug!("Saved credentials to {:?}", self.path);
        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        std::fs::remove_file(&self.path).ok();
        debug!("Cleared credentials in {:?}", self.path);
        Ok(())
    }
}

#[async_trait]
impl TmrCredStore for PlaintextCredStore {
    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.client_secret = Some(secret.into());
        dbg!(&creds);
        self.save_creds(creds)?;
        debug!("Saved client secret to {:?}", self.path);
        Ok(())
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        let client_secret = self.load_creds()?.and_then(|c| c.client_secret);
        debug!("Loaded client secret from {:?}", self.path);
        Ok(client_secret)
    }
}
