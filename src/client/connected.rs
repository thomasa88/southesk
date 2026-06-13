// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use std::borrow::Cow;

use rmcp::model::{CallToolRequestParams, CallToolResult, JsonObject};
#[cfg(feature = "__dev")]
use serde::Serialize;
use serde::de::DeserializeOwned;
use tracing::debug;
#[cfg(feature = "__dev")]
use tracing::{info, warn};

use crate::{
    ClientCallError,
    types::{
        AccountHoldings, AccountIdentifiers, CreateTradeTicketResult, HoldingsSelector,
        InstrumentIdentifiers, ModifyWatchlistResult, TradeCurrency, TradeTicketArgs, Watchlist,
        WatchlistInfo,
    },
};

use super::{Client, Connected};

/// # Montrose API methods
///
/// Each method maps directly to a Montrose MCP tool of the same name.
impl Client<Connected> {
    /// Returns holdings for either one account (when
    /// [`HoldingsSelector::AccountId`] is provided) or all accessible accounts.
    /// Each account includes
    /// [`currency_positions`](AccountHoldings::currency_positions): the
    /// account's multi-currency cash balances with the amount available for
    /// purchase per currency. An empty
    /// [`currency_positions`](AccountHoldings::currency_positions) list means
    /// the account holds cash only in its main currency (see
    /// [`summary`](AccountHoldings::summary)). Use
    /// [`get_user_accounts`](Self::get_user_accounts) first to find valid
    /// account IDs.
    pub async fn get_holdings(
        &self,
        selection: HoldingsSelector,
    ) -> Result<Vec<AccountHoldings>, ClientCallError> {
        let mut args = serde_json::Map::new();
        args.insert(
            "accountId".to_string(),
            match selection {
                HoldingsSelector::AccountId(account_id) => Some(account_id.to_string()),
                HoldingsSelector::All => None,
            }
            .into(),
        );
        self.api_call("get_holdings", Some(args)).await
    }

    /// Returns all user accounts with stable account IDs and display names. Use
    /// this tool to discover valid account IDs before calling
    /// [`get_holdings`](Self::get_holdings) for a specific account.
    pub async fn get_user_accounts(&self) -> Result<Vec<AccountIdentifiers>, ClientCallError> {
        self.api_call("get_user_accounts", None).await
    }

    /// Creates a pre-filled trade ticket URL for the Montrose app. Specify side
    /// (Buy/Sell), quantity or amount, and an instrument identifier. Use
    /// orderbookId directly when known, since it is the safest identifier. If
    /// you only know a ticker or name and it may be ambiguous, call
    /// [`search_instruments`](Self::search_instruments) first to find the
    /// correct orderbookId, then call
    /// [`create_trade_ticket`](Self::create_trade_ticket). Returns a URL that
    /// opens the trade ticket in the Montrose app with the order details
    /// pre-filled.
    pub async fn create_trade_ticket(
        &self,
        args: TradeTicketArgs,
    ) -> Result<reqwest::Url, ClientCallError> {
        if args.currency == TradeCurrency::Account && args.account_id.is_none() {
            return Err(ClientCallError::InvalidArguments(
                "Either currency or account_id must be set".to_string(),
            ));
        }
        let arg_map = match serde_json::to_value(args) {
            Ok(serde_json::Value::Object(map)) => map,
            Ok(_) => {
                return Err(ClientCallError::InvalidArguments(
                    "Could not convert args to JSON object".to_string(),
                ));
            }
            Err(_) => {
                return Err(ClientCallError::InvalidArguments(
                    "Could not convert args to JSON".to_string(),
                ));
            }
        };
        self.api_call::<CreateTradeTicketResult>("create_trade_ticket", Some(arg_map))
            .await
            .map(|res| res.url)
    }

    /// Searches instruments by ticker or name and returns matching
    /// orderbookIds, tickers, and names. Use this tool before
    /// [`create_trade_ticket`](Self::create_trade_ticket) when multiple
    /// instruments have similar names.
    ///
    /// Seems to return at most 9 results.
    pub async fn search_instruments(
        &self,
        query: &str,
    ) -> Result<Vec<InstrumentIdentifiers>, ClientCallError> {
        let mut arg_map = serde_json::Map::new();
        arg_map.insert("query".to_string(), query.into());
        self.api_call("search_instruments", Some(arg_map)).await
    }

    /// Returns the authenticated user's watchlists with their ID, name, and the
    /// number of instruments on each list. Use [`get_watchlist`](Self::get_watchlist) with a listId to
    /// read the instruments on a specific watchlist.
    pub async fn get_watchlists(&self) -> Result<Vec<WatchlistInfo>, ClientCallError> {
        self.api_call("get_watchlists", None).await
    }

    /// Returns the instruments on a single watchlist, identified by `list_id`.
    /// Each instrument is enriched with its orderbookId, ticker and name. Use
    /// [`get_watchlists`](Self::get_watchlists) first to discover valid listIds.
    pub async fn get_watchlist(&self, list_id: u64) -> Result<Watchlist, ClientCallError> {
        let mut arg_map = serde_json::Map::new();
        arg_map.insert("listId".to_string(), list_id.into());
        self.api_call("get_watchlist", Some(arg_map)).await
    }

    /// Creates a new watchlist with the given name for the authenticated user.
    /// If a watchlist with the same name already exists, returns that existing
    /// watchlist.
    #[doc(alias = "create_or_get_watchlist")]
    pub async fn create_watchlist(&self, name: &str) -> Result<WatchlistInfo, ClientCallError> {
        let mut arg_map = serde_json::Map::new();
        arg_map.insert("name".to_string(), name.into());
        self.api_call("create_watchlist", Some(arg_map)).await
    }

    /// Adds one or more instruments to an existing watchlist by orderbookId.
    /// Use [`search_instruments`](Self::search_instruments) to find the correct
    /// orderbookId for a ticker or name. Instruments already on the watchlist
    /// are silently skipped.
    pub async fn add_to_watchlist(
        &self,
        list_id: u64,
        orderbook_ids: &[u64],
    ) -> Result<ModifyWatchlistResult, ClientCallError> {
        let mut arg_map = serde_json::Map::new();
        arg_map.insert("listId".to_string(), list_id.into());
        arg_map.insert("orderbookIds".to_string(), orderbook_ids.into());
        self.api_call("add_to_watchlist", Some(arg_map)).await
    }

    /// Removes one or more instruments from a watchlist by orderbookId. Returns
    /// the orderbookIds that were actually removed; orderbookIds that were not
    /// on the watchlist are silently ignored and excluded from the response.
    pub async fn remove_from_watchlist(
        &self,
        list_id: u64,
        orderbook_ids: &[u64],
    ) -> Result<ModifyWatchlistResult, ClientCallError> {
        let mut arg_map = serde_json::Map::new();
        arg_map.insert("listId".to_string(), list_id.into());
        arg_map.insert("orderbookIds".to_string(), orderbook_ids.into());
        self.api_call("remove_from_watchlist", Some(arg_map)).await
    }
}

/// # Raw API calls
///
/// The following methods provides raw API access to the MCP. They can be used
/// when the corresponding APIs are not yet implement in [`Client`].
impl Client<Connected> {
    /// Calls the specified MCP tool with the given arguments.
    ///
    /// This function can be used before a new MCP API tool has been added to
    /// southesk.
    ///
    /// Set `T` as a type that implements `Deserialize` that matches the
    /// expected response format. You can use [`serde_json::Value`] when the
    /// format is unknown.
    ///
    /// This is a low-level API call. Prefer using the higher-level methods when
    /// available.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use southesk::ClientBuilder;
    /// # tokio_test::block_on(
    /// # async {
    /// # let client = ClientBuilder::new("My Montrose client").build().await?;
    /// # let client = client.connect().await?;
    /// use southesk::raw::json_object;
    ///
    /// let result: serde_json::Value = client
    ///     .raw_tool_call::<serde_json::Value>(
    ///         "get_holdings",
    ///         Some(json_object!({"accountId": "771c4286-991c-48aa-965e-c7dd62e31735"})))
    ///     .await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    ///
    /// ```no_run
    /// # use southesk::ClientBuilder;
    /// # tokio_test::block_on(
    /// # async {
    /// # let client = ClientBuilder::new("My Montrose client").build().await?;
    /// # let client = client.connect().await?;
    /// let mut args = serde_json::Map::new();
    /// args.insert(
    ///     "accountId".to_string(),
    ///     "771c4286-991c-48aa-965e-c7dd62e31735".into(),
    /// );
    /// let result: serde_json::Value = client
    ///     .raw_tool_call::<serde_json::Value>("get_holdings", Some(args))
    ///     .await?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// # });
    /// ```
    pub async fn raw_tool_call<T: DeserializeOwned>(
        &self,
        tool_name: impl Into<Cow<'static, str>>,
        args: Option<JsonObject>,
    ) -> Result<T, ClientCallError> {
        self.api_call(tool_name, args).await
    }
}

// Helpers
impl Client<Connected> {
    /// Calls the specified MCP tool with the given arguments.
    async fn api_call<T: DeserializeOwned>(
        &self,
        tool_name: impl Into<Cow<'static, str>>,
        args: Option<JsonObject>,
    ) -> Result<T, ClientCallError> {
        let req = CallToolRequestParams::new(tool_name);
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
}

// Development utilities
#[cfg(feature = "__dev")]
#[doc(hidden)]
impl Client<Connected> {
    /// Fetches and prints available tools and prompts from the server.
    /// Used for southesk development.
    ///
    /// # Panics
    /// Panics if writing to the result string fails.
    pub async fn introspect(&self) -> String {
        fn parse<T: Serialize, E: std::error::Error>(
            d: &str,
            res: Result<Vec<T>, E>,
            json_result: &mut serde_json::Map<String, serde_json::Value>,
        ) {
            match res {
                Ok(items) => {
                    info!("Available {d}: {}", items.len());
                    json_result.insert(d.to_string(), serde_json::to_value(&items).unwrap());
                }
                Err(e) => {
                    warn!("Error fetching {d}: {e}");
                    json_result.insert(d.to_string(), serde_json::Value::Null);
                }
            }
        }

        let mut json_result = serde_json::Map::new();

        parse(
            "tools",
            self.state.client.peer().list_all_tools().await,
            &mut json_result,
        );
        parse(
            "prompts",
            self.state.client.peer().list_all_prompts().await,
            &mut json_result,
        );
        parse(
            "resources",
            self.state.client.peer().list_all_resources().await,
            &mut json_result,
        );
        serde_json::to_string_pretty(&json_result).unwrap()
    }
}

fn parse_result<T: DeserializeOwned>(res: &CallToolResult) -> Result<T, ClientCallError> {
    let text = &res
        .content
        .first()
        .ok_or(ClientCallError::parse_err("No content element in response"))?
        .raw
        .as_text()
        .ok_or(ClientCallError::parse_err("No raw text in response"))?
        .text;
    if res.is_error.unwrap_or(false) {
        return Err(ClientCallError::McpError(format!(
            "Error from server: {text}"
        )));
    }
    serde_json::from_str::<T>(text).map_err(|e| ClientCallError::ParseError {
        msg: format!("Failed to parse response text: {text}"),
        source: Some(e.into()),
    })
}
