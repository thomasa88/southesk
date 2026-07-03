// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! # southesk
//!
//! southesk is a library for creating clients for the [Montrose
//! MCP](https://www.montrose.io/mcp) API.
//!
//! [`Client`] provides the main interface to the library.
//!
//! # Quickstart
//! To use `southesk`, add it as a dependency along with an async runtime:
//!
//! ```bash
//! > cargo add southesk
//! > cargo add tokio -F rt-multi-thread
//! ```
//!
//! Then you can create a client and make API calls:
//!
//! ```no_run
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let montrose = southesk::ClientBuilder::new("My Montrose Client")
//!         .build()
//!         .await?;
//!     let montrose = montrose.connect().await?;
//!
//!     let accounts = montrose.get_user_accounts().await?;
//!     dbg!(&accounts);
//!
//!     montrose.disconnect().await;
//!
//!     Ok(())
//! }
//! ```

pub use client::{Client, ClientBuilder, Connected, Disconnected};
pub mod auth_handler;
mod client;
pub mod cred_store;
pub mod error;
#[cfg(feature = "low-api")]
pub mod low_level;
#[cfg(feature = "raw-api")]
pub mod raw;
#[cfg(not(feature = "raw-api"))]
mod raw;
pub mod types;

// Re-export dependencies that are part of the public interface
pub use reqwest;
pub use reqwest::Url;
pub use rust_decimal;
pub use rust_decimal::Decimal;
pub use uuid;
pub use uuid::Uuid;
