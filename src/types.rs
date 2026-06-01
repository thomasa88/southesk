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

    /// ISO 4217 currency code (e.g. "SEK", "USD", "EUR") for the amount.
    pub currency: TradeCurrency,

    /// The instrument to trade
    #[serde(flatten)]
    pub instrument: Instrument,
}

/// Specifies how much of an instrument to trade.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TradeVolume {
    /// Amount (money) to trade. If the user explicitly specifies a currency
    /// (e.g. "10 000 SEK", "500 USD"), pass it via the
    /// [`TradeTicketArgs::currency`] parameter.
    #[serde(rename = "amount")]
    Amount(Decimal),
    /// Number of shares to trade.
    Quantity(Decimal),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum TradeCurrency {
    /// Use the account's main currency.
    #[default]
    Account,
    /// Use the currency given by the ISO 4217 code.
    ///
    /// Examples: `SEK`, `USD`, `EUR`.
    Code(String),
}

impl Serialize for TradeCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        // From the create_trade_ticket tool documentation:
        // "If the user explicitly specifies a currency
        // (e.g. "10 000 SEK", "500 USD"), pass it via the currency parameter;
        // otherwise leave currency unset and the account's main currency will
        // be used."
        match self {
            Self::Account => serializer.serialize_none(),
            Self::Code(code) => {
                if code.len() != 3 {
                    return Err(S::Error::custom("currency code should be 3 letters"));
                }
                serializer.serialize_some(code)
            }
        }
    }
}

impl<'de> Deserialize<'de> for TradeCurrency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_option(TradeCurrencyVisitor)
    }
}

struct TradeCurrencyVisitor;

impl<'de> serde::de::Visitor<'de> for TradeCurrencyVisitor {
    type Value = TradeCurrency;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an ISO 4217 code of length 3")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TradeCurrency::Account)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = String::deserialize(deserializer)?;
        if code.len() != 3 {
            return Err(serde::de::Error::custom(format!(
                "Currency code should be 3 letters: {code}"
            )));
        }
        Ok(TradeCurrency::Code(code))
    }
}

/// Specifies the instrument to trade.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Instrument {
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
pub struct InstrumentIdentifiers {
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
    pub items: Vec<InstrumentIdentifiers>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveFromWatchlistResult {
    pub list_id: u64,
    pub orderbook_ids: Vec<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trade_currency_serialization() {
        assert_eq!(
            serde_json::to_string(&TradeCurrency::Account).unwrap(),
            "null"
        );
        assert_eq!(
            serde_json::from_str::<TradeCurrency>("null").unwrap(),
            TradeCurrency::Account
        );

        assert_eq!(
            serde_json::to_string(&TradeCurrency::Code("SEK".into())).unwrap(),
            "\"SEK\""
        );
        assert_eq!(
            serde_json::from_str::<TradeCurrency>("\"USD\"").unwrap(),
            TradeCurrency::Code("USD".into())
        );
        assert!(serde_json::from_str::<TradeCurrency>("\"WRONG_LEN\"").is_err());
        assert!(serde_json::from_str::<TradeCurrency>("\"\"").is_err());
    }
}
