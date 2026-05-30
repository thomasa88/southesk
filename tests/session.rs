use std::time::Duration;

use southesk::ClientBuilder;
use tokio::time::sleep;
use tracing::warn;

#[tokio::test]
#[ignore = "takes very long time"]
async fn access_token_timeout() -> anyhow::Result<()> {
    // TODO: Read the actual timeout time from the token and use it.

    tracing_subscriber::fmt().init();

    let montrose = ClientBuilder::new("southesk tests")
        .cred_user("access_token_timeout_test")
        .build()
        .await?
        .connect()
        .await?;
    warn!("MCP use attempt 1");
    montrose.get_user_accounts().await?;

    sleep(Duration::from_secs(3600)).await;

    warn!("MCP use attempt 2");
    montrose.get_user_accounts().await?;

    sleep(Duration::from_secs(25 * 3600)).await;

    warn!("MCP use attempt 3");
    montrose.get_user_accounts().await?;

    Ok(())
}
