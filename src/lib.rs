// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! [`TmrClient`] provides the main interface to the library.

pub use client::TmrClient;
pub use result::TmrCallError;
pub use result::TmrConnectError;

pub use rust_decimal::Decimal;
pub use uuid::Uuid;

mod client;
mod cred_store;
pub mod auth_callback;
mod result;
pub mod types;
