// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use rmcp::model::{CallToolRequestParams, CallToolResult, JsonObject};
use serde::de::DeserializeOwned;
use std::fmt::Write;
use tracing::debug;

use crate::{TmrCallError, types};

use super::{Connected, TmrClient};

impl TmrClient<Connected> {
    /// Returns holdings for either one account (when accountId is provided) or
    /// all accessible accounts. Use get_user_accounts first to find valid account
    /// IDs.
    pub async fn get_holdings(
        &self,
        selection: types::HoldingsSelector,
    ) -> Result<Vec<types::Account>, TmrCallError> {
        let mut args = serde_json::Map::new();
        args.insert(
            "accountId".to_string(),
            match selection {
                types::HoldingsSelector::AccountId(account_id) => Some(account_id.to_string()),
                types::HoldingsSelector::All => None,
            }
            .into(),
        );
        self.api_call("get_holdings", Some(args)).await
    }

    /// Returns all user accounts with stable account IDs and display names. Use
    /// this tool to discover valid account IDs before calling get_holdings for a
    /// specific account.
    pub async fn get_user_accounts(&self) -> Result<Vec<types::AccountInfo>, TmrCallError> {
        self.api_call("get_user_accounts", None).await
    }

    /// Creates a pre-filled trade ticket URL for the Montrose app. Specify side
    /// (Buy/Sell), quantity or amount, and an instrument identifier. Use
    /// orderbookId directly when known, since it is the safest identifier. If
    /// you only know a ticker or name and it may be ambiguous, call
    /// search_instruments first to find the correct orderbookId, then call
    /// create_trade_ticket. Returns a URL that opens the trade ticket in the
    /// Montrose app with the order details pre-filled.
    pub async fn create_trade_ticket(
        &self,
        args: types::TradeTicketArgs,
    ) -> Result<reqwest::Url, TmrCallError> {
        let arg_map = match serde_json::to_value(args) {
            Ok(serde_json::Value::Object(map)) => map,
            Ok(_) => {
                return Err(TmrCallError::InvalidArguments(
                    "Could not convert args to JSON object".to_string(),
                ));
            }
            Err(_) => {
                return Err(TmrCallError::InvalidArguments(
                    "Could not convert args to JSON".to_string(),
                ));
            }
        };
        self.api_call::<types::CreateTradeTicketResult>("create_trade_ticket", Some(arg_map))
            .await
            .map(|res| res.url)
    }

    /// Searches instruments by ticker or name and returns matching
    /// orderbookIds, tickers, and names. Use this tool before
    /// create_trade_ticket when multiple instruments have similar names.
    pub async fn search_instruments(
        &self,
        query: &str,
    ) -> Result<Vec<types::SearchInstrumentResultItem>, TmrCallError> {
        let mut arg_map = serde_json::Map::new();
        arg_map.insert("query".to_string(), query.into());
        self.api_call("search_instruments", Some(arg_map)).await
    }

    /// Calls the specified MCP tool with the given arguments.
    async fn api_call<T: DeserializeOwned>(
        &self,
        tool: &str,
        args: Option<JsonObject>,
    ) -> Result<T, TmrCallError> {
        let req = CallToolRequestParams::new(tool.to_owned());
        let req = if let Some(args) = args {
            req.with_arguments(args)
        } else {
            req
        };
        debug!("Call request: {:#?}", req);
        let res = self.state.client.call_tool(req).await?;
        debug!("Call response: {:#?}", res);
        parse_result::<T>(&res)
    }

    /// Fetches and prints available tools and prompts from the server.
    /// Used for TmrClient development.
    pub async fn introspect(&self) -> String {
        let mut result = String::new();
        writeln!(result, "Fetching available tools from server...").unwrap();

        match self.state.client.peer().list_all_tools().await {
            Ok(tools) => {
                writeln!(result, "Available tools: {}", tools.len()).unwrap();
                for tool in tools {
                    writeln!(
                        result,
                        "- {} ({})\n{:#?}\n{:#?}\n",
                        tool.name,
                        tool.description.unwrap_or_default(),
                        tool.input_schema,
                        tool.output_schema,
                    )
                    .unwrap();
                }
            }
            Err(e) => {
                writeln!(result, "Error fetching tools: {e}").unwrap();
            }
        }

        writeln!(result, "Fetching available prompts from server...").unwrap();

        match self.state.client.peer().list_all_prompts().await {
            Ok(prompts) => {
                writeln!(result, "Available prompts: {}", prompts.len()).unwrap();
                for prompt in prompts {
                    writeln!(result, "- {}", prompt.name).unwrap();
                }
            }
            Err(e) => {
                writeln!(result, "Error fetching prompts: {e}").unwrap();
            }
        }
        result
    }
}

fn parse_result<T: DeserializeOwned>(res: &CallToolResult) -> Result<T, TmrCallError> {
    let text = &res
        .content
        .first()
        .ok_or(TmrCallError::parse_err("No content element in response"))?
        .raw
        .as_text()
        .ok_or(TmrCallError::parse_err("No raw text in response"))?
        .text;
    if res.is_error.unwrap_or(false) {
        return Err(TmrCallError::McpError(format!("Error from server: {text}")));
    }
    serde_json::from_str::<T>(text).map_err(|e| TmrCallError::ParseError {
        msg: format!("Failed to parse response text: {text}"),
        source: Some(e.into()),
    })
}
