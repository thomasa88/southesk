// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use std::{io::Write, net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use axum::{
    Router,
    extract::{Query, State},
    response::Html,
    routing::get,
};
use reqwest::Url;
use serde::Deserialize;
use tokio::{
    net::TcpListener,
    sync::{Mutex, oneshot},
};
use tracing::{debug, error, info};

use crate::result::TmrConnectError;

const BROWSER_CALLBACK_HTML: &str = include_str!("res/default_callback.html");

#[async_trait]
pub trait AuthHandler {
    /// The URL that the OAuth server will redirect to after authentication. It
    /// is passed as part of the OAuth authorization request.
    fn redirect_uri(&self) -> &str;
    /// Requests the user to authenticate by visiting the given `auth_url` and
    /// waits for the OAuth callback to be received.
    async fn authenticate(&self, auth_url: &str) -> Result<AuthGrant, TmrConnectError>;
}

/// An OAuth callback handler that opens the user's web browser and listens for
/// the callback on a local HTTP server.
pub struct BrowserAuth {
    listen_addr: String,
    cb_tx: Arc<Mutex<Option<oneshot::Sender<AuthGrant>>>>,
}

#[derive(Clone)]
struct BrowserAuthAppState {
    cb_tx: Arc<Mutex<Option<oneshot::Sender<AuthGrant>>>>,
}

/// The parameters received in the OAuth callback URL, containing the
/// authorization code and state (CSRF token).
#[derive(Debug, Deserialize)]
pub struct AuthGrant {
    /// The authorization code to exchange for an access token
    pub code: String,
    /// CSRF token sent by the client and now returned by the server
    pub state: String,
}

async fn browser_callback_handler(
    Query(params): Query<AuthGrant>,
    State(state): State<BrowserAuthAppState>,
) -> Html<String> {
    debug!("Received callback: {params:?}");
    // Anyone waiting for the value?
    if let Some(cb_tx) = state.cb_tx.lock().await.take() {
        cb_tx.send(params).ok();
    }
    Html(BROWSER_CALLBACK_HTML.to_string())
}

impl BrowserAuth {
    pub async fn new() -> Result<Self, TmrConnectError> {
        // 0 means to bind to a random available port
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| TmrConnectError::AuthError {
                msg: format!("Failed to bind callback server: {e}"),
                source: Some(e.into()),
            })?;
        let addr = listener
            .local_addr()
            .map_err(|e| TmrConnectError::AuthError {
                msg: format!("Failed to get address of callback server: {e}"),
                source: Some(e.into()),
            })?;
        // The MCP server does not like an IP as the host in the callback server (HTTP/2 403)
        let listen_addr = format!("http://localhost:{}/callback", addr.port());

        // let (cb_tx, cb_rx) = watch::channel::<Option<oneshot::Sender<AuthGrant>>>(None);
        let cb_rx = Arc::new(Mutex::new(None));

        let app_state = BrowserAuthAppState {
            cb_tx: cb_rx.clone(),
        };

        let app = Router::new()
            .route("/callback", get(browser_callback_handler))
            .with_state(app_state);

        info!("Listening for callback at: {}", listen_addr);

        tokio::spawn(async move {
            let result = axum::serve(listener, app).await;

            if let Err(e) = result {
                error!("Callback server error: {e}");
            }
        });

        Ok(BrowserAuth {
            listen_addr,
            cb_tx: cb_rx,
        })
    }
}

#[async_trait]
impl AuthHandler for BrowserAuth {
    fn redirect_uri(&self) -> &str {
        &self.listen_addr
    }

    async fn authenticate(&self, auth_url: &str) -> Result<AuthGrant, TmrConnectError> {
        let (cb_tx, cb_rx) = oneshot::channel();
        *self.cb_tx.lock().await = Some(cb_tx);
        eprintln!("Opening authorization page in browser: {auth_url}");
        eprintln!("Please complete the authentication in the opened browser window.");
        webbrowser::open(auth_url).ok();
        info!("Waiting for browser callback");
        let params = cb_rx.await.map_err(|e| TmrConnectError::AuthError {
            msg: format!("Failed to receive callback parameters: {e}"),
            source: Some(e.into()),
        })?;
        eprintln!("Authentication completed.");
        Ok(params)
    }
}

/// An OAuth callback handler that works in a console.
///
/// It prints the authorization URL and waits for the user to paste the redirect
/// URL.
pub struct ConsoleAuth {
    redirect_base_url: String,
}

impl Default for ConsoleAuth {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleAuth {
    pub fn new() -> Self {
        ConsoleAuth {
            redirect_base_url: "http://localhost:7878/callback".to_string(),
        }
    }
}

#[async_trait]
impl AuthHandler for ConsoleAuth {
    fn redirect_uri(&self) -> &str {
        &self.redirect_base_url
    }

    async fn authenticate(&self, auth_url: &str) -> Result<AuthGrant, TmrConnectError> {
        eprintln!("\nOpen the following URL in your browser to authenticate:\n");
        eprintln!("  {auth_url}\n");
        eprintln!("After completing authentication, your browser will redirect to a URL");
        eprintln!(
            "starting with '{}?...' (the page won't load).",
            self.redirect_base_url
        );
        eprint!("\nPaste that full redirect URL here: ");
        std::io::stdout().flush().ok();

        let redirect_url = tokio::task::spawn_blocking(|| {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).ok();
            line.trim().to_owned()
        })
        .await
        .map_err(|e| TmrConnectError::AuthError {
            msg: format!("Failed to read redirect URL from stdin: {e}"),
            source: None,
        })?;

        let parsed = Url::parse(&redirect_url).map_err(|e| TmrConnectError::AuthError {
            msg: format!("Invalid redirect URL '{redirect_url}': {e}"),
            source: None,
        })?;

        let grant = serde_urlencoded::from_str(parsed.query().unwrap_or("")).map_err(|e| {
            TmrConnectError::AuthError {
                msg: format!(
                    "Failed to parse query parameters from redirect URL '{redirect_url}': {e}"
                ),
                source: Some(e.into()),
            }
        })?;

        Ok(grant)
    }
}
