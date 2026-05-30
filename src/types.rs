// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Montrose API types.
//!
//! The types correspond to the types used by the Montrose MCP API. Some have
//! been slightly adapted for better ergonomics in Rust.

use reqwest::Url;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_with::{NoneAsEmptyString, serde_as};
use uuid::Uuid;

// TODO: Generate types from tool input and output JSON schemas?

/// Used to select which holdings to fetch when calling
/// [`get_holdings`](crate::Client::get_holdings).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HoldingsSelector {
    All,
    AccountId(Uuid),
}

/// Full account information, including holdings.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: Uuid,
    pub account_number: String,
    #[serde_as(as = "NoneAsEmptyString")]
    pub account_name: Option<String>,
    pub currency: String,
    pub summary: AccountSummary,
    pub positions: Vec<Position>,
}

/// Summary of the account holdings.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    /// Value of the investments in the account
    pub total_market_value: Decimal,
    /// Amount available for purchase
    pub available_for_purchase: Decimal,
    /// Total value of the account. The sum of investments and cash.
    pub total_value: Decimal,
    /// Currency of the account
    pub currency: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub instrument_name: String,
    pub ticker: String,
    pub orderbook_id: u64,
    pub possible_orderbook_ids: Vec<u64>,
    /// Number of shares
    pub quantity: Decimal,
    /// Value of the position (quantity * price)
    pub market_value: InstrumentValue,
    pub unrealized_result: InstrumentValue,
    pub unrealized_result_percent: Decimal,
    pub instrument_currency: String,
    /// Exchange rate (instrument_currency / account_currency)
    pub fx_rate: Decimal,
}

/// Value of an instrument in different currencies.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentValue {
    /// Value in the account currency
    pub account_currency: Decimal,
    /// Value in the instrument currency
    pub instrument_currency: Decimal,
}

/// Various identifiers that can be used to identify an account.
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentifiers {
    /// Example: `d075c5d4-222f-4ba9-b973-10bb9aeea705`
    pub account_id: Uuid,
    /// Example: `1234567`
    pub account_number: String,
    /// Account name, as set by the user. It is not guaranteed to be unique, as
    /// multiple accounts can have the same name.
    #[serde_as(as = "NoneAsEmptyString")]
    pub account_name: Option<String>,
}

/// Arguments for creating a trade ticket, used with
/// [`create_trade_ticket`](crate::Client::create_trade_ticket).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeTicketArgs {
    /// The side of the order: Buy or Sell.
    pub side: TradeSide,

    /// Optional account ID. Use
    /// [`get_user_accounts`](crate::Client::get_user_accounts) to find valid
    /// account IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Uuid>,

    /// Optional price for the order.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price: Option<Decimal>,

    /// How much of the instrument to trade.
    #[serde(flatten)]
    pub volume: TradeVolume,

    /// Optional ISO 4217 currency code (e.g. "SEK", "USD", "EUR") for the
    /// amount. Set this only when the user explicitly states a currency. When
    /// omitted, the account's main currency is used.
    pub currency: Option<String>,

    /// The instrument to trade
    #[serde(flatten)]
    pub instrument: TradeInstrument,
}

/// Specifies how much of an instrument to trade.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TradeVolume {
    /// Amount (money) to trade. If the user explicitly specifies a currency
    /// (e.g. "10 000 SEK", "500 USD"), pass it via the
    /// [`TradeTicketArgs::currency`] parameter; otherwise leave currency unset
    /// and the account's main currency will be used.
    #[serde(rename = "amount")]
    Amount(Decimal),
    /// Number of shares to trade.
    Quantity(Decimal),
}

/// Specifies the instrument to trade.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TradeInstrument {
    /// Optional instrument name (string) to search for the instrument. This is
    /// a convenience identifier and may be ambiguous; use [`search_instruments`](crate::Client::search_instruments) to
    /// find the correct orderbookId when needed.
    Name(String),
    /// Optional orderbookId (int) to identify the instrument directly. This is the safest identifier and should be preferred when known or after using [`search_instruments`](crate::Client::search_instruments).
    OrderbookId(u64),
    /// Optional ticker (string) to identify the instrument by ticker symbol,
    /// e.g. \"VOLV B\". This is a convenience identifier and may be ambiguous;
    /// use [`search_instruments`](crate::Client::search_instruments) to find the correct orderbookId when needed.
    Ticker(String),
}

// Whether to buy or sell an instrument.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
pub enum TradeSide {
    #[default]
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateTradeTicketResult {
    pub url: Url,
}

/// Various identifiers that can be used to identify a trade instrument.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradeInstrumentInfo {
    /// Instrument name
    pub name: String,
    /// Instrument order book ID
    pub orderbook_id: u64,
    /// Instrument ticker
    pub ticker: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchlistInfo {
    pub list_id: u64,
    pub name: String,
    /// Number of instruments in the watchlist
    pub orderbook_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Watchlist {
    pub list_id: u64,
    pub name: String,
    pub items: Vec<TradeInstrumentInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromWatchlistResult {
    pub list_id: u64,
    pub orderbook_ids: Vec<u64>,
}
