// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use rmcp::transport::{AuthError, CredentialStore};

/// An extension of [`CredentialStore`] that allows saving and loading the
/// client secret.
///
/// Since the MCP API creates the client secret when the client is registered
/// and then requires it for token refreshes, it needs to be stored.
///
/// The tmr-client implementation assumes that the state is shared between all
/// credential stores initialized with the same configuration. It can be
/// implemented using a shared file, for example.
pub trait TmrCredStore: CredentialStore + Clone {
    async fn save_client_secret(&self, secret: impl Into<String>) -> Result<(), AuthError>;
    async fn load_client_secret(&self) -> Result<Option<String>, AuthError>;
}
