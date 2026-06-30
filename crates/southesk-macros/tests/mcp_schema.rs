// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use std::{borrow::Cow, marker::PhantomData};

use rust_decimal::dec;
use serde::de::DeserializeOwned;
use southesk::error::ClientCallError;
use southesk::raw::JsonObject;
use southesk_macros::mcp_schema;

struct Client<S> {
    _state: PhantomData<S>,
}

struct Connected;

mcp_schema!("tests/tools.json");

impl<S> Client<S> {
    async fn api_call<T: DeserializeOwned>(
        &self,
        tool_name: impl Into<Cow<'static, str>>,
        _args: Option<JsonObject>,
    ) -> Result<T, ClientCallError> {
        let tool_name = tool_name.into();
        match tool_name.as_ref() {
            "simple_tool" => Ok(serde_json::from_str(
                r#"{
                    "output": 42
                }"#,
            )
            .unwrap()),
            _ => panic!("Unexpected tool: {}", &tool_name),
        }
    }
}

#[tokio::test]
async fn call_simple_tool() {
    let client = Client::<Connected> {
        _state: PhantomData,
    };
    assert_eq!(
        client.low_simple_tool("input_str").await.unwrap(),
        dec!(42)
    );
}

#[test]
pub fn check_expanded() {
    macrotest::expand("tests/expand/*.rs");
}
