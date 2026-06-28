// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Demonstrates how to call the low-level MCP API directly.

use anyhow::Context;
use rust_decimal::dec;
use southesk::low_level::types::CreateTradeTicketArgs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,southesk=info,show_data=info".to_string().into()),
        )
        .init();

    let montrose = southesk::ClientBuilder::new("southesk sample")
        .build()
        .await?;
    let montrose = montrose.connect().await?;

    let account = &montrose.low_get_user_accounts().await?[0];
    let holdings: Vec<_> = montrose
        .low_get_holdings(Some(&account.account_id.to_string()))
        .await?
        .into_iter()
        .filter(|h| h.account_type.as_deref() == Some("ISK"))
        .collect();

    montrose
        .low_create_trade_ticket(CreateTradeTicketArgs {
            side: "Buy",
            orderbook_id: holdings
                .first()
                .and_then(|h| h.positions.first())
                .context("No existing position in first account")?
                .orderbook_id,
            ticker: None,
            name: None,
            quantity: Some(dec!(1.0)),
            amount: None,
            price: None,
            account_id: None,
            currency: None,
        })
        .await?;

    montrose.disconnect().await;

    Ok(())
}
