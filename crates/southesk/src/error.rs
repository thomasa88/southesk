// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Errors that are not tied to specific modules.

use crate::auth_handler;

/// Errors that occur when doing calls to the MCP API.
#[derive(Debug, thiserror::Error)]
pub enum ClientCallError {
    /// Problems when communicating with the MCP server.
    #[error("MCP service communication error")]
    CommunicationError(#[from] rmcp::ServiceError),
    /// The MCP service returned an error response.
    #[error("error response from MCP service: {0}")]
    McpError(String),
    /// Failure to parse the response from the MCP service.
    #[error("failed to parse response: {msg}")]
    ParseError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    /// The arguments provided to the tool function are invalid.
    #[error("invalid arguments")]
    InvalidArguments(String),
}

impl ClientCallError {
    pub(crate) fn parse_err(msg: impl Into<String>) -> ClientCallError {
        Self::ParseError {
            msg: msg.into(),
            source: None,
        }
    }
}

/// Errors that occur when connecting the client.
#[derive(Debug, thiserror::Error)]
pub enum ClientConnectError {
    /// A step of the authentication process failed.
    #[error("authentication failed: {msg}")]
    AuthError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
    /// A step of the authentication process failed, inside the
    /// [authentication handler](auth_handler).
    ///
    /// [`AuthFlowError::Aborted`](auth_handler::AuthFlowError::Aborted)
    ///  can indicate that the user cancelled or failed the authentication
    /// steps.
    #[error("authentication handler failed")]
    AuthHandlerError(#[from] auth_handler::AuthFlowError),
    /// Failed to set up a connection to the MCP servic.
    #[error("connection failed: {msg}")]
    ConnectionError {
        msg: String,
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

pub(crate) trait MapAuthToConnectError<T> {
    fn to_connect_auth_err(self, msg: impl Into<String>) -> Result<T, ClientConnectError>;
}

impl<T> MapAuthToConnectError<T> for Result<T, rmcp::transport::AuthError> {
    fn to_connect_auth_err(self, msg: impl Into<String>) -> Result<T, ClientConnectError> {
        self.map_err(|e| ClientConnectError::AuthError {
            msg: msg.into(),
            source: Some(Box::new(e)),
        })
    }
}

/// An error occured when building the client.
#[derive(Debug, thiserror::Error)]
#[error("failed to build client: {msg}")]
pub struct ClientBuildError {
    pub msg: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}
