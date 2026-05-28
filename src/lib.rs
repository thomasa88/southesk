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
pub mod cred_store;
mod result;
pub mod types;
