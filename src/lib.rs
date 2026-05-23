// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! [`TmrClient`] provides the main interface to the library.

pub use client::TmrClient;
pub use client::TmrClientBuilder;
pub use result::TmrCallError;
pub use result::TmrConnectError;

pub use rust_decimal::Decimal;
pub use uuid::Uuid;

pub mod auth_handler;
mod client;
mod result;
pub mod types;

#[cfg(feature = "keyring")]
mod keyring_cred_store;
#[cfg(not(feature = "keyring"))]
mod plain_cred_store;
