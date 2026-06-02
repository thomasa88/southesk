// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Shows the user's accounts and holdings.

use southesk::{Decimal, rust_decimal::dec, types::HoldingsSelector};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,southesk=info,show_data=info".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let montrose = southesk::ClientBuilder::new("southesk sample")
        .build()
        .await?;
    let montrose = montrose.connect().await?;

    println!("-------- Accounts --------");
    let accounts = montrose.get_user_accounts().await?;
    for account in &accounts {
        println!(
            "Account: {} {} {}",
            account.account_id,
            account.account_number,
            account.account_name.as_deref().unwrap_or("")
        );
    }

    println!();
    println!("-------- Holdings --------");
    let accounts = montrose.get_holdings(HoldingsSelector::All).await?;
    for account in accounts {
        println!(
            "Account: {} {}",
            account.account_number,
            account.account_name.as_deref().unwrap_or("")
        );
        let currency = &account.summary.currency;
        println!("  Account ID: {}", account.account_id);
        println!("  Currency: {}", account.summary.currency);
        println!(
            "  Total market value: {:.2} {}",
            account.summary.total_market_value, currency
        );
        println!(
            "  Available for purchase: {:.2} {}",
            account.summary.available_for_purchase, currency
        );
        println!(
            "  Total value: {:.2} {}",
            account.summary.total_value, currency
        );

        {
            let weight = account
                .summary
                .available_for_purchase
                .checked_div(account.summary.total_value)
                .unwrap_or(Decimal::ZERO)
                * dec!(100.0);
            println!("  Position: Cash");
            println!(
                "    Value: {:.2} {} ({:.2}%)",
                account.summary.available_for_purchase, currency, weight
            );
        }
        let mut positions = account.positions;
        positions.sort_by(|a, b| a.instrument_name.cmp(&b.instrument_name));
        for position in positions {
            let weight = position
                .market_value
                .account_currency
                .checked_div(account.summary.total_value)
                .unwrap_or(Decimal::ZERO)
                * dec!(100.0);
            println!("  Position: {}", position.instrument_name);
            println!("    Order book ID: {}", position.orderbook_id);
            println!("    Ticker: {}", position.ticker);
            println!("    Quantity: {:.2}", position.quantity);
            println!(
                "    Value: {:.2} {} ({:.2}%)",
                position.market_value.account_currency, currency, weight
            );
        }
    }

    Ok(())
}
