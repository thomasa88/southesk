// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Demonstrates how to do correct clean-up, even when errors occur.

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

    let res = run(&montrose).await;

    // Clean-up that always runs since the call above did not use `?`.
    montrose.disconnect().await;

    res
}

async fn run(montrose: &southesk::Client<southesk::Connected>) -> anyhow::Result<()> {
    let _ = montrose.get_user_accounts().await?;

    // Trigger an error
    montrose
        .raw_tool_call::<serde_json::Value>("tool_that_does_not_exist", None)
        .await?;

    Ok(())
}
