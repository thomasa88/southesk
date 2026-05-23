// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use rmcp::{RoleClient, model::InitializeRequestParams, service::RunningService};

use crate::{
    auth_handler::{AuthHandler, BrowserAuth},
    result::TmrBuildError,
};

mod connection;
mod tools;

/// The Montrose MCP client
///
/// The client must first be connected, then the Montrose API functions can be
/// used. Build a client using [`TmrClientBuilder`].
///
/// [`TmrClient<Connected>`] provides the available API functions.
///
/// The user will automatically be requested to authenticate if there is no
/// valid cached OAuth token.
///
/// # Examples
/// ```no_run
/// # use tmr_client::TmrClient;
/// # tokio_test::block_on(
/// # async {
/// let montrose = TmrClient::new("My Montrose client");
/// let montrose = montrose.connect().await?;
///
/// let accounts = montrose.get_user_accounts().await?;
/// for account in &accounts {
///     dbg!(account);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
pub struct TmrClient<S: State = Disconnected> {
    client_name: String,
    lib_dirs: etcetera::app_strategy::Xdg,
    auth_handler: Box<dyn AuthHandler>,
    state: S,
}

mod private {
    pub trait Sealed {}
}
pub trait State: private::Sealed {}

pub struct Disconnected;
pub struct Connected {
    client: RunningService<RoleClient, InitializeRequestParams>,
}

impl private::Sealed for Disconnected {}
impl private::Sealed for Connected {}

impl State for Disconnected {}
impl State for Connected {}

/// The [`TmrClient`] builder.
pub struct TmrClientBuilder {
    client_name: String,
    auth_handler: Option<Box<dyn AuthHandler>>,
}

impl TmrClientBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            client_name: name.into(),
            auth_handler: None,
        }
    }

    /// Overrides how the user is requested to authenticate.
    /// 
    /// [`BrowserAuth`] is used by default.
    pub fn auth_handler(&mut self, handler: impl AuthHandler + 'static) -> &mut Self {
        self.auth_handler = Some(Box::new(handler));
        self
    }

    pub async fn build(self) -> Result<TmrClient<Disconnected>, TmrBuildError> {
        let auth_handler = match self.auth_handler {
            Some(handler) => handler,
            None => Box::new(
                BrowserAuth::new()
                    .await
                    .map_err(|e| TmrBuildError::BuildError {
                        msg: e.to_string(),
                        source: Some(Box::new(e)),
                    })?,
            ),
        };

        let lib_dirs = etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
            top_level_domain: "".to_string(),
            author: "thomasa88".to_string(),
            app_name: "tmr-client".to_string(),
        })
        .map_err(|e| TmrBuildError::BuildError {
            msg: e.to_string(),
            source: Some(Box::new(e)),
        })?;
        Ok(TmrClient {
            client_name: self.client_name,
            lib_dirs,
            auth_handler,
            state: Disconnected {},
        })
    }
}
