// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use etcetera::AppStrategy;
use tmr_client::cred_store::plaintext_cred_store::PlaintextCredStore;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,tmr_client=info,show_data=info".to_string().into()),
        )
        .init();

    let dirs = etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
        top_level_domain: "".to_string(),
        author: "".to_string(),
        app_name: "tmr-client-sample".to_string(),
    })?;
    let creds_file = dirs
        .state_dir()
        .unwrap_or_else(|| dirs.data_dir())
        .join("credentials.json");

    let montrose = tmr_client::TmrClientBuilder::new("TMR Client Sample")
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
