// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in a plaintext JSON file in
//! the user's config directory.

use std::path::PathBuf;
use tracing::debug;

use async_trait::async_trait;
use etcetera::AppStrategy;
use rmcp::transport::{AuthError, StoredCredentials};

use super::{CombinedStoredCreds, TmrCredStore};

#[derive(Debug, Clone)]
pub struct PlaintextCredStore {
    dirs: etcetera::app_strategy::Xdg,
    filename: PathBuf,
}

impl PlaintextCredStore {
    /// Creates a new plaintext credential store in the computer user's config
    /// directory.
    pub fn new(dirs: etcetera::app_strategy::Xdg) -> Self {
        Self {
            filename: Self::create_filename(&dirs, "user_not_set"),
            dirs,
        }
    }

    fn create_filename(dirs: &etcetera::app_strategy::Xdg, user: &str) -> PathBuf {
        dirs.config_dir().join(format!("{user}_credentials.json"))
    }

    fn load_creds(&self) -> Result<Option<CombinedStoredCreds>, AuthError> {
        if !self.filename.exists() {
            return Ok(None);
        }
        let file = std::fs::File::open(&self.filename).map_err(|e| {
            AuthError::InternalError(format!("Failed to open credentials file: {e}"))
        })?;
        let creds: CombinedStoredCreds = serde_json::from_reader(file).map_err(|e| {
            AuthError::InternalError(format!("Failed to deserialize credentials: {e}"))
        })?;
        Ok(Some(creds))
    }

    fn save_creds(&self, creds: CombinedStoredCreds) -> Result<(), AuthError> {
        if let Some(parent) = self.filename.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AuthError::InternalError(format!(
                    "Failed to create directories for credentials file {:?}: {e}",
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
        let file = options.open(&self.filename).map_err(|e| {
            AuthError::InternalError(format!(
                "Failed to create credentials file {:?}: {e}",
                self.filename
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
impl TmrCredStore for PlaintextCredStore {
    fn set_user(&mut self, user: &str) {
        self.filename = Self::create_filename(&self.dirs, user);
    }

    async fn save_client_secret(&self, secret: &str) -> Result<(), AuthError> {
        let mut creds = self.load_creds()?.unwrap_or_default();
        creds.client_secret = Some(secret.into());
        dbg!(&creds);
        self.save_creds(creds)?;
        debug!("Saved client secret to {:?}", self.filename);
        Ok(())
    }

    async fn load_client_secret(&self) -> Result<Option<String>, AuthError> {
        let client_secret = self.load_creds()?.and_then(|c| c.client_secret);
        debug!("Loaded client secret from {:?}", self.filename);
        Ok(client_secret)
    }

    fn dyn_clone(&self) -> Box<dyn TmrCredStore> {
        Box::new((*self).clone())
    }
}
