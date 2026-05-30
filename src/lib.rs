// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! [`Client`] provides the main interface to the library.

pub use client::Client;
pub use client::ClientBuilder;
pub use result::ClientCallError;
pub use result::ClientConnectError;

pub use rust_decimal::Decimal;
pub use uuid::Uuid;

pub mod auth_handler;
mod client;
pub mod cred_store;
mod result;
pub mod types;
