// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! # southesk
//!
//! southesk is a library for creating clients for the [Montrose
//! MCP](https://www.montrose.io/mcp) API.
//!
//! [`Client`] provides the main interface to the library.

pub use client::{Client, ClientBuilder, Connected, Disconnected};
pub use result::{ClientBuildError, ClientCallError, ClientConnectError};
pub mod auth_handler;
mod client;
pub mod cred_store;
mod result;
pub mod types;

// Re-export dependencies that are part of the public interface
pub use reqwest;
pub use reqwest::Url;
pub use rust_decimal;
pub use rust_decimal::Decimal;
pub use uuid;
pub use uuid::Uuid;

pub mod raw {
    pub use rmcp::object as json_object;
}
