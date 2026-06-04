use std::time::Duration;

use southesk::ClientBuilder;
use tokio::time::sleep;
#[cfg(feature = "__dev")]
use tokio::time::timeout;
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

#[cfg(feature = "__dev")]
#[tokio::test]
#[ignore = "takes very long time"]
async fn force_token_refresh() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    // Make sure we have a valid token
    let montrose = ClientBuilder::new("southesk tests")
        .cred_user("token_refresh_test")
        .build()
        .await?
        .connect()
        .await?;

    // Test that the token works
    montrose.get_user_accounts().await?;

    sleep(Duration::from_secs(1)).await;

    // Refresh the token
    let montrose = ClientBuilder::new("southesk tests")
        .cred_user("token_refresh_test")
        // Don't wait for any interactive auth
        .no_auth()
        .dev_force_token_refresh()
        .build()
        .await?;

    warn!("Connecting to MCP with forced token refresh");
    let montrose = timeout(Duration::from_secs(5), montrose.connect()).await??;

    // Test that the refreshed token works
    montrose.get_user_accounts().await?;

    Ok(())
}
