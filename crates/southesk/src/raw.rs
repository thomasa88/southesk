// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Tools to interact with the raw MCP API.

pub use rmcp::model::JsonObject;
#[cfg(feature = "raw-api")]
pub use rmcp::object as json_object;
