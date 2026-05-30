// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use std::io::{self, Write};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,southesk=info,introspect=info".to_string().into()),
        )
        // Log to stderr, so that logs and output can be separated
        .with(tracing_subscriber::fmt::layer().with_writer(io::stderr))
        .init();

    let arg = std::env::args().nth(1);

    let mut montrose_builder = southesk::ClientBuilder::new("southesk sample");
    if arg.as_deref() == Some("--console-auth") {
        montrose_builder =
            montrose_builder.auth_handler(southesk::auth_handler::ConsoleAuth::new());
    }
    let montrose = montrose_builder.build().await?;
    let montrose = montrose.connect().await?;

    io::stdout().write_all(montrose.introspect().await.as_bytes())?;

    Ok(())
}
