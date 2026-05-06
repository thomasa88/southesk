// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn usage() {
    println!("Usage: search-instrument <instrument name or ticker>");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,create_balanced_trade=info".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 || args[1] == "--help" {
        usage();
        return Ok(());
    }
    let instrument_name_or_ticker = args.get(1).unwrap();

    let montrose = tmr_client::TmrClient::new("TMR Client Sample");
    let montrose = montrose.connect().await?;

    let instruments = montrose
        .search_instruments(instrument_name_or_ticker)
        .await?;
    for instrument in instruments {
        println!(
            "{:5} {:15} {}",
            instrument.orderbook_id, instrument.ticker, instrument.name
        );
    }

    Ok(())
}
