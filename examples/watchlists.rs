// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Shows how to manipulate watchlists.

use anyhow::ensure;
use std::time::Duration;
use tokio::time::sleep;

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

    println!();
    println!("-------- Watchlists --------");
    let watchlists = montrose.get_watchlists().await?;
    dbg!(&watchlists);

    for watchlist in &watchlists {
        dbg!(montrose.get_watchlist(watchlist.list_id).await?);
        // Let's be nice to the server
        sleep(Duration::from_secs(1)).await;
    }

    println!();
    println!("-------- Create watchlist --------");
    let created_list = montrose.create_watchlist("southesk list!").await?;
    dbg!(&created_list);

    let montglobe_matches = montrose.search_instruments("MONTGLOBE").await?;
    ensure!(
        montglobe_matches.len() == 1,
        "Expected exactly one instrument matching 'MONTGLOBE'"
    );

    println!();
    println!("-------- Add instrument to watchlist --------");
    let montglobe = &montglobe_matches[0];
    let added = montrose
        .add_to_watchlist(created_list.list_id, &[montglobe.orderbook_id])
        .await?;
    dbg!(&added);

    println!();
    println!("-------- Remove instrument from watchlist --------");
    let montglobe = &montglobe_matches[0];
    let removed = montrose
        .remove_from_watchlist(created_list.list_id, &[montglobe.orderbook_id])
        .await?;
    dbg!(&removed);

    Ok(())
}
