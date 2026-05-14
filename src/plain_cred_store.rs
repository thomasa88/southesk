// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! A credential store that saves the credentials in a plaintext JSON file in
//! the user's config directory.

use std::path::PathBuf;
use tracing::debug;

use async_trait::async_trait;
use etcetera::AppStrategy;
use rmcp::transport::{AuthError, StoredCredentials};

pub struct PlainCredStore {
    filename: PathBuf,
}

impl PlainCredStore {
    pub fn new(dirs: &etcetera::app_strategy::Xdg) -> Self {
        let filename = dirs.config_dir().join("credentials.json");
        Self { filename }
    }
}

#[async_trait]
impl rmcp::transport::CredentialStore for PlainCredStore {
    async fn load(&self) -> Result<Option<StoredCredentials>, AuthError> {
        if !self.filename.exists() {
            return Ok(None);
        }
        let file = std::fs::File::open(&self.filename).map_err(|e| {
            AuthError::InternalError(format!("Failed to open credentials file: {e}"))
        })?;
        let creds: StoredCredentials = serde_json::from_reader(file).map_err(|e| {
            AuthError::InternalError(format!("Failed to deserialize credentials: {e}"))
        })?;
        debug!("Loaded credentials from {:?}", self.filename);
        Ok(Some(creds))
    }

    async fn save(&self, credentials: StoredCredentials) -> Result<(), AuthError> {
        if let Some(parent) = self.filename.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AuthError::InternalError(format!(
                    "Failed to create directories for credentials file {:?}: {e}",
                    self.filename
                ))
            })?;
        }
        let file = std::fs::File::create(&self.filename).map_err(|e| {
            AuthError::InternalError(format!(
                "Failed to create credentials file {:?}: {e}",
                self.filename
            ))
        })?;
        serde_json::to_writer_pretty(file, &credentials).map_err(|e| {
            AuthError::InternalError(format!("Failed to serialize credentials: {e}"))
        })?;
        debug!("Saved credentials to {:?}", self.filename);
        Ok(())
    }

    async fn clear(&self) -> Result<(), AuthError> {
        std::fs::remove_file(&self.filename).ok();
        debug!("Cleared credentials in {:?}", self.filename);
        Ok(())
    }
}
