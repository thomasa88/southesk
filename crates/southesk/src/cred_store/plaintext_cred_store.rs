// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in a plaintext JSON file in
//! the user's config directory.

use std::{
    io::{Read, Write},
    path::PathBuf,
};
use tracing::debug;

use async_trait::async_trait;
use rmcp::transport::{AuthError, StoredCredentials};

use super::{CombinedStoredCreds, FullCredStore};

/// A credential store that saves the credentials in a plaintext JSON file in
/// the user's config directory.
#[derive(Debug, Clone)]
pub struct PlaintextCredStore {
    filename: PathBuf,
}

impl PlaintextCredStore {
    /// Creates a new plaintext credential with the given filename.
    ///
    /// Use different paths for different Montrose accounts.
    pub fn new(filename: impl Into<PathBuf>) -> Self {
        Self {
            filename: filename.into(),
        }
    }

    fn load_creds(&self) -> Result<Option<CombinedStoredCreds>, AuthError> {
        if !self.filename.exists() {
            return Ok(None);
        }
        let mut file = std::fs::File::open(&self.filename).map_err(|e| {
            AuthError::InternalError(format!("failed to open credentials file: {e}"))
        })?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).map_err(|e| {
            AuthError::InternalError(format!(
                "failed to read credentials from {}: {}",
                self.filename.display(),
                e
            ))
        })?;
        super::decode_json_creds(&buf)
    }

    fn save_creds(&self, creds: CombinedStoredCreds) -> Result<(), AuthError> {
        if let Some(parent) = self.filename.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AuthError::InternalError(format!(
                    "failed to create directories for credentials file {:?}: {e}",
                    self.filename
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
        let mut file = options.open(&self.filename).map_err(|e| {
            AuthError::InternalError(format!(
                "failed to create credentials file {:?}: {e}",
                self.filename
            ))
        })?;
        file.write_all(&super::encode_json_creds(&creds)?)
            .map_err(|e| {
                AuthError::InternalError(format!(
                    "failed to write credentials to {}: {}",
                    self.filename.display(),
                    e
                ))
            })?;
        Ok(())
    }
}

#[async_trait]
impl rmcp::transport::CredentialStore for PlaintextCredStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        let creds = self.load_creds()?.and_then(|c| c.rmcp_creds);
        if creds.is_some() {
            debug!("Loaded credentials from {:?}", self.filename);
        }
        Ok(creds)
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.rmcp_creds = Some(credentials);
        self.save_creds(creds)?;
        debug!("Saved credentials to {:?}", self.filename);
        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        std::fs::remove_file(&self.filename).ok();
        debug!("Cleared credentials in {:?}", self.filename);
        Ok(())
    }
}

#[async_trait]
impl FullCredStore for PlaintextCredStore {
    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.client_secret = Some(secret.into());
        self.save_creds(creds)?;
        debug!("Saved client secret to {:?}", self.filename);
        Ok(())
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        let client_secret = self.load_creds()?.and_then(|c| c.client_secret);
        debug!("Loaded client secret from {:?}", self.filename);
        Ok(client_secret)
    }
}
