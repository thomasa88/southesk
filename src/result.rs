// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

/// Errors that occur when doing calls to the MCP API.
#[derive(Debug, thiserror::Error)]
pub enum ClientCallError {
    #[error("MCP service communication error")]
    CommunicationError(#[from] rmcp::ServiceError),
    #[error("Error response from MCP service: {0}")]
    McpError(String),
    #[error("Failed to parse response: {msg}")]
    ParseError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    #[error("Invalid arguments")]
    InvalidArguments(String),
}

impl ClientCallError {
    pub fn parse_err(msg: impl Into<String>) -> ClientCallError {
        Self::ParseError {
            msg: msg.into(),
            source: None,
        }
    }
}

/// Errors that occur when connecting the client.
#[derive(Debug, thiserror::Error)]
pub enum ClientConnectError {
    #[error("Authentication failed: {msg}")]
    AuthError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    #[error("Connection failed: {msg}")]
    ConnectionError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

pub(crate) trait MapAuthToConnectError<T> {
    fn to_connect_err(self, msg: impl Into<String>) -> Result<T, ClientConnectError>;
}

impl<T> MapAuthToConnectError<T> for Result<T, rmcp::transport::AuthError> {
    fn to_connect_err(self, msg: impl Into<String>) -> Result<T, ClientConnectError> {
        self.map_err(|e| ClientConnectError::AuthError {
            msg: msg.into(),
            source: Some(Box::new(e)),
        })
    }
}

/// Errors that occur when building the client.
#[derive(Debug, thiserror::Error)]
pub enum ClientBuildError {
    #[error("Failed to build client: {msg}")]
    BuildError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
