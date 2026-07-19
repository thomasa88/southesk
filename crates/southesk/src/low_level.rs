// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Montrose Low-level API

use crate::Client;
use crate::Connected;
use crate::error::ClientCallError;
use southesk_macros::mcp_schema;

mcp_schema!("data/montrose-api/api.json");
