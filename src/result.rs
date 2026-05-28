// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

#[derive(Debug, thiserror::Error)]
pub enum TmrCallError {
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

impl TmrCallError {
    pub fn parse_err(msg: impl Into<String>) -> TmrCallError {
        Self::ParseError {
            msg: msg.into(),
            source: None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TmrConnectError {
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
    fn to_connect_err(self, msg: impl Into<String>) -> Result<T, TmrConnectError>;
}

impl<T> MapAuthToConnectError<T> for Result<T, rmcp::transport::AuthError> {
    fn to_connect_err(self, msg: impl Into<String>) -> Result<T, TmrConnectError> {
        self.map_err(|e| TmrConnectError::AuthError {
            msg: msg.into(),
            source: Some(Box::new(e)),
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TmrBuildError {
    #[error("Failed to build client: {msg}")]
    BuildError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
