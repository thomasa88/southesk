// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Demonstrates how to call the MCP API directly.

use southesk::raw::json_object;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,southesk=info,show_data=info".to_string().into()),
        )
        .init();

    let montrose = southesk::ClientBuilder::new("southesk raw API sample")
        .build()
        .await?;
    let montrose = montrose.connect().await?;

    let result: serde_json::Value = montrose
        .raw_tool_call::<serde_json::Value>("get_holdings", Some(json_object!({"accountId": null})))
        .await?;
    dbg!(result);

    montrose.disconnect().await;

    Ok(())
}
