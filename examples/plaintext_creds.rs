// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

//! Shows how to use the plaintext credential store, which saves the OAuth
//! client secret in a text file.

use etcetera::AppStrategy;
use southesk::cred_store::plaintext_cred_store::PlaintextCredStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,southesk=info,show_data=info".to_string().into()),
        )
        .init();

    let dirs = etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
        top_level_domain: "".to_string(),
        author: "".to_string(),
        app_name: "southesk-sample".to_string(),
    })?;
    let creds_file = dirs
        .state_dir()
        .unwrap_or_else(|| dirs.data_dir())
        .join("credentials.json");

    let montrose = southesk::ClientBuilder::new("southesk sample")
        .cred_store(PlaintextCredStore::new(&creds_file))
        .build()
        .await?;
    let _montrose = montrose.connect().await?;

    println!(
        "Credentials stored as plaintext in {}",
        creds_file.display()
    );

    Ok(())
}
