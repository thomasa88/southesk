// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use rmcp::{RoleClient, model::InitializeRequestParams, service::RunningService};

use crate::{
    auth_handler::{AuthHandler, BrowserAuth},
    cred_store::{FullCredStore, SharedCredStore, keyring_cred_store::KeyringCredStore},
    result::ClientBuildError,
};

mod connected;
mod disconnected;

/// The Montrose MCP client
///
/// The client must first be connected, then the Montrose API functions can be
/// used. Build a client using [`ClientBuilder`].
///
/// [`Client<Connected>`] provides the available API functions.
///
/// The user will automatically be requested to authenticate if there is no
/// valid cached OAuth token.
///
/// # Examples
/// ```no_run
/// # use southesk::ClientBuilder;
/// # tokio_test::block_on(
/// # async {
/// let montrose = ClientBuilder::new("My Montrose Client").build().await?;
/// let montrose = montrose.connect().await?;
///
/// let accounts = montrose.get_user_accounts().await?;
/// for account in &accounts {
///     dbg!(account);
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// # });
/// ```
#[derive(Debug)]
pub struct Client<S: State = Disconnected> {
    client_name: String,
    auth_handler: Option<Box<dyn AuthHandler>>,
    cred_store: SharedCredStore,
    state: S,
}

mod private {
    pub trait Sealed {}
}
pub trait State: private::Sealed {}

#[derive(Debug)]
pub struct Disconnected;

#[derive(Debug)]
pub struct Connected {
    client: RunningService<RoleClient, InitializeRequestParams>,
}

impl private::Sealed for Disconnected {}
impl private::Sealed for Connected {}

impl State for Disconnected {}
impl State for Connected {}

const DEFAULT_CRED_USER: &str = "user";

/// The [`Client`] builder.
#[derive(Debug)]
pub struct ClientBuilder {
    client_name: String,
    auth_handler: Option<Box<dyn AuthHandler>>,
    interactive_auth: bool,
    cred_user: Option<String>,
    cred_store: Option<SharedCredStore>,
}

impl ClientBuilder {
    /// Creates a new builder for [`Client`].
    ///
    /// `client_name` is used to identify the client towards the MCP service. It
    /// is recommended to name it after your application.
    pub fn new(client_name: impl Into<String>) -> Self {
        Self {
            client_name: client_name.into(),
            auth_handler: None,
            interactive_auth: true,
            cred_user: None,
            cred_store: None,
        }
    }

    /// Overrides how the user is requested to authenticate.
    ///
    /// [`BrowserAuth`] is used by default.
    pub fn auth_handler(mut self, handler: impl AuthHandler + 'static) -> Self {
        self.auth_handler = Some(Box::new(handler));
        self
    }

    /// Disables initiating new interactive authentications (through
    /// [`AuthHandler`]).
    ///
    /// The client will only rely on existing OAuth tokens (from a previous
    /// session). This can be useful for long-running background programs.
    /// [`Client::connect`] will fail if the server denies the existing OAuth
    /// credentials.
    pub fn no_auth(mut self) -> Self {
        self.interactive_auth = false;
        self
    }

    /// Sets the user identifier used for the default credential storage.
    ///
    /// This option can be used if the current computer user needs to store
    /// credentials for multiple Montrose accounts or sessions (e.g. for
    /// testing).
    ///
    /// This option is not valid if a custom credential store is provided with
    /// [`ClientBuilder::cred_store`].
    pub fn cred_user(mut self, user: impl Into<String>) -> Self {
        self.cred_user = Some(user.into());
        self
    }

    /// Overrides the credential store used to store the user's OAuth credentials
    pub fn cred_store(mut self, cred_store: impl FullCredStore + 'static) -> Self {
        self.cred_store = Some(SharedCredStore::new(cred_store));
        self
    }

    pub async fn build(self) -> Result<Client<Disconnected>, ClientBuildError> {
        let auth_handler = if self.interactive_auth {
            Some(match self.auth_handler {
                Some(handler) => handler,
                None => Box::new(BrowserAuth::new().await.map_err(|e| {
                    ClientBuildError::BuildError {
                        msg: e.to_string(),
                        source: Some(Box::new(e)),
                    }
                })?),
            })
        } else {
            None
        };

        let cred_store = match self.cred_store {
            Some(store) => {
                if self.cred_user.is_some() {
                    return Err(ClientBuildError::BuildError {
                        msg: "Cannot specify both a custom credential store and a credential user"
                            .to_string(),
                        source: None,
                    });
                }
                store
            }
            None => {
                let cred_user = self.cred_user.unwrap_or(DEFAULT_CRED_USER.to_string());
                SharedCredStore::new({
                    #[cfg(feature = "keyring")]
                    {
                        KeyringCredStore::new(&self.client_name, &cred_user).map_err(|e| {
                            ClientBuildError::BuildError {
                                msg: "Failed to create keyring credential store".to_string(),
                                source: Some(Box::new(e)),
                            }
                        })?
                    }
                    #[cfg(not(feature = "keyring"))]
                    {
                        let client_dirs =
                            etcetera::choose_app_strategy(etcetera::AppStrategyArgs {
                                top_level_domain: "".to_string(),
                                author: "".to_string(),
                                app_name: self.client_name.clone(),
                            })
                            .map_err(|e| {
                                ClientBuildError::BuildError {
                                    msg: e.to_string(),
                                    source: Some(Box::new(e)),
                                }
                            })?;
                        PlaintextCredStore::new(&client_dirs)
                    }
                })
            }
        };

        Ok(Client {
            client_name: self.client_name,
            auth_handler,
            cred_store,
            state: Disconnected,
        })
    }
}
