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

const CALLBACK_HTML: &str = include_str!("res/default_callback.html");

#[async_trait]
pub trait AuthCallbackHandler {
    async fn new() -> Result<Box<Self>, TmrConnectError>;
    fn get_listen_addr(&self) -> &str;
    async fn wait_for_callback(self, auth_url: &str) -> Result<AuthCallback, TmrConnectError>;
}

pub struct DefaultAuthCallbackHandler {
    listener: TcpListener,
    listen_addr: String,
}

#[derive(Clone)]
struct AppState {
    code_sender: Arc<Mutex<Option<oneshot::Sender<AuthCallback>>>>,
}

#[derive(Debug, Deserialize)]
pub struct AuthCallback {
    pub code: String,
    pub state: String,
}

async fn callback_handler(
    Query(params): Query<AuthCallback>,
    State(state): State<AppState>,
) -> Html<String> {
    debug!("Received callback: {params:?}");
    if let Some(sender) = state.code_sender.lock().await.take() {
        let _ = sender.send(params);
    }
    Html(CALLBACK_HTML.to_string())
}

#[async_trait]
impl AuthCallbackHandler for DefaultAuthCallbackHandler {
    async fn new() -> Result<Box<Self>, TmrConnectError> {
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

        Ok(Box::new(DefaultAuthCallbackHandler {
            listener,
            listen_addr,
        }))
    }

    fn get_listen_addr(&self) -> &str {
        &self.listen_addr
    }

    async fn wait_for_callback(self, auth_url: &str) -> Result<AuthCallback, TmrConnectError> {
        let (code_sender, code_receiver) = oneshot::channel::<AuthCallback>();

        let app_state = AppState {
            code_sender: Arc::new(Mutex::new(Some(code_sender))),
        };

        let app = Router::new()
            .route("/callback", get(callback_handler))
            .with_state(app_state);

        info!("Listening for callback at: {}", self.listen_addr);

        let listener = self.listener;
        tokio::spawn(async move {
            let result = axum::serve(listener, app).await;

            if let Err(e) = result {
                error!("Callback server error: {e}");
            }
        });

        eprintln!("Opening authorization page in browser: {auth_url}");
        eprintln!("Please complete the authentication in the opened browser window.");
        webbrowser::open(auth_url).ok();
        info!("Waiting for browser callback");
        let params = code_receiver
            .await
            .map_err(|e| TmrConnectError::AuthError {
                msg: format!("Failed to receive callback parameters: {e}"),
                source: Some(e.into()),
            })?;
        eprintln!("Authenticaton completed.");
        Ok(params)
    }
}

pub struct ConsoleAuthHandler {
    redirect_uri: String,
}

#[async_trait]
impl AuthCallbackHandler for ConsoleAuthHandler {
    async fn new() -> Result<Box<Self>, TmrConnectError> {
        Ok(Box::new(ConsoleAuthHandler {
            redirect_uri: "http://localhost:7878/callback".to_string(),
        }))
    }

    fn get_listen_addr(&self) -> &str {
        &self.redirect_uri
    }

    async fn wait_for_callback(
        self,
        auth_url: &str,
    ) -> Result<AuthCallback, TmrConnectError> {
        eprintln!("\nOpen the following URL in your browser to authenticate:\n");
        eprintln!("  {auth_url}\n");
        eprintln!("After completing authentication, your browser will redirect to a URL");
        eprintln!("starting with '{}?...' (the page won't load).", self.redirect_uri);
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

        let parsed = Url::parse(&redirect_url).map_err(|e| {
            TmrConnectError::AuthError {
                msg: format!("Invalid redirect URL '{redirect_url}': {e}"),
                source: None,
            }
        })?;

        let mut code = None;
        let mut state = None;
        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "code" => code = Some(value.into_owned()),
                "state" => state = Some(value.into_owned()),
                _ => {}
            }
        }

        let code =
            code.ok_or_else(|| TmrConnectError::AuthError {
                msg: "Missing 'code' parameter in redirect URL".to_string(),
                source: None,
            })?;
        let state =
            state.ok_or_else(|| TmrConnectError::AuthError {
                msg: "Missing 'state' parameter in redirect URL".to_string(),
                source: None,
            })?;

        Ok(AuthCallback { code, state })
    }
}
