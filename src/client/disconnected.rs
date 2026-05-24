// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use rmcp::{
    RoleClient, ServiceExt,
    model::{ClientInfo, InitializeRequestParams},
    service::RunningService,
    transport::{
        AuthClient, AuthorizationManager, StreamableHttpClientTransport, auth::OAuthState,
        streamable_http_client::StreamableHttpClientTransportConfig,
    },
};
use tracing::{debug, info};

#[cfg(feature = "keyring")]
use crate::keyring_cred_store::KeyringCredStore;
#[cfg(not(feature = "keyring"))]
use crate::plain_cred_store::PlainCredStore;
use crate::{auth_handler, result::TmrConnectError};

use super::{Connected, Disconnected, TmrClient};

const MCP_SERVER_URL: &str = "https://mcp.montrose.io";

impl TmrClient<Disconnected> {
    pub async fn connect(self) -> Result<TmrClient<Connected>, TmrConnectError> {
        let auth_mgr = self.authenticate().await?;

        let mut mcp_client_res = self.init_mcp_client(auth_mgr).await;

        if let Err(rmcp::service::ClientInitializeError::TransportError {
            error: dyn_transport_err,
            context: _,
        }) = &mcp_client_res
        {
            debug!("Transport error: {dyn_transport_err:#?}");
            // Try again if it was an authorization error
            if Self::is_auth_required_error(dyn_transport_err) {
                info!("Authentication required error encountered");
                info!("Starting new authorization flow");
                let auth_mgr = self.authenticate_new_auth().await?;
                mcp_client_res = self.init_mcp_client(auth_mgr).await;
            }
        }

        let mcp_client = mcp_client_res.map_err(|e| TmrConnectError::ConnectionError {
            msg: "Failed to connect to MCP server".to_string(),
            source: Some(e.into()),
        })?;

        info!("Successfully connected to MCP server");

        Ok(TmrClient {
            client_name: self.client_name,
            lib_dirs: self.lib_dirs,
            auth_handler: self.auth_handler,
            cred_user: self.cred_user,
            state: Connected { client: mcp_client },
        })
    }

    fn is_auth_required_error(dyn_transport_err: &rmcp::transport::DynamicTransportError) -> bool {
        let http_error = dyn_transport_err
            .error
            .downcast_ref::<rmcp::transport::streamable_http_client::StreamableHttpError<
            reqwest::Error,
        >>();
        matches!(
            http_error,
            Some(
                rmcp::transport::streamable_http_client::StreamableHttpError::Auth(
                    rmcp::transport::AuthError::AuthorizationRequired,
                ) | rmcp::transport::streamable_http_client::StreamableHttpError::AuthRequired(
                    rmcp::transport::streamable_http_client::AuthRequiredError { .. }
                )
            )
        )
    }

    async fn init_mcp_client(
        &self,
        auth_mgr: AuthorizationManager,
    ) -> Result<
        RunningService<RoleClient, InitializeRequestParams>,
        rmcp::service::ClientInitializeError,
    > {
        let auth_client = AuthClient::new(reqwest::Client::default(), auth_mgr);
        let transport = StreamableHttpClientTransport::with_client(
            auth_client,
            StreamableHttpClientTransportConfig::with_uri(MCP_SERVER_URL),
        );
        let client_service = ClientInfo::default();
        client_service.serve(transport).await
    }

    async fn authenticate(&self) -> Result<AuthorizationManager, TmrConnectError> {
        debug!("Using MCP server URL: {}", MCP_SERVER_URL);

        info!("Establishing authorized connection to MCP server...");
        // Cannot convert an OAuthState into an AuthorizationManager, as it
        // initially isn't in the Authorized state. So we start with an
        // AuthorizationManager in case we already have usable credentials.
        let mut auth_mgr = AuthorizationManager::new(MCP_SERVER_URL)
            .await
            .map_err(|e| TmrConnectError::AuthError {
                msg: "Failed to initialize authorization manager".to_string(),
                source: Some(e.into()),
            })?;
        auth_mgr.set_credential_store(self.create_cred_store()?);
        // The authorization manager automatically does a token refresh if
        // needed. See REFRESH_BUFFER_SECS in rmcp.
        let initialized =
            auth_mgr
                .initialize_from_store()
                .await
                .map_err(|e| TmrConnectError::AuthError {
                    msg: "Failed to initialize authorization manager from credential store"
                        .to_string(),
                    source: Some(e.into()),
                })?;
        if initialized {
            info!(
                "Initialized authorization manager for \"{}\" from credential store",
                self.cred_user
            );
            return Ok(auth_mgr);
        }

        info!(
            "No credentials found in store for \"{}\". Starting new authorization flow.",
            self.cred_user
        );
        self.authenticate_new_auth().await
    }

    async fn authenticate_new_auth(&self) -> Result<AuthorizationManager, TmrConnectError> {
        let mut oauth_state = OAuthState::new(MCP_SERVER_URL, None).await.map_err(|e| {
            TmrConnectError::AuthError {
                msg: "Failed to initialize OAuth state".to_string(),
                source: Some(e.into()),
            }
        })?;
        oauth_state.set_credential_store(self.create_cred_store()?);

        // oauth: Empty scope will let the server select
        let wanted_scopes = &["mcp"];
        debug!("Requesting scopes: {:?}", wanted_scopes);

        let redirect_uri = self.auth_handler.redirect_uri();
        debug!("Using redirect URI: {}", redirect_uri);
        oauth_state
            .start_authorization(wanted_scopes, redirect_uri, Some(&self.client_name))
            .await
            .map_err(|e| TmrConnectError::AuthError {
                msg: "Failed to start authorization".to_string(),
                source: Some(e.into()),
            })?;

        let auth_url =
            oauth_state
                .get_authorization_url()
                .await
                .map_err(|e| TmrConnectError::AuthError {
                    msg: "Failed to get authorization URL".to_string(),
                    source: Some(e.into()),
                })?;
        debug!("Auth URL: {}", auth_url);

        info!("Waiting for authorization code...");
        let auth_handler::AuthGrant {
            code: auth_code,
            state: csrf_token,
        } = self.auth_handler.authenticate(&auth_url).await?;
        info!("Received authorization code: {}", auth_code);

        info!("Exchanging authorization code for access token...");
        oauth_state
            .handle_callback(&auth_code, &csrf_token)
            .await
            .map_err(|e| TmrConnectError::AuthError {
                msg: "Failed to handle authorization callback".to_string(),
                source: Some(e.into()),
            })?;
        info!("Successfully obtained access token");

        info!("Authorization successful! Access token obtained.");

        let (client_id, Some(_token_response)) =
            oauth_state
                .get_credentials()
                .await
                .map_err(|e| TmrConnectError::AuthError {
                    msg: "Failed to get credentials from OAuth state".to_string(),
                    source: Some(e.into()),
                })?
        else {
            return Err(TmrConnectError::AuthError {
                msg: "No credentials obtained from OAuth state".to_string(),
                source: None,
            });
        };
        debug!("Obtained client id: {}", client_id);

        let auth_mgr =
            oauth_state
                .into_authorization_manager()
                .ok_or_else(|| TmrConnectError::AuthError {
                    msg: "Failed to convert OAuth state into authorization manager".to_string(),
                    source: None,
                })?;

        Ok(auth_mgr)
    }

    fn create_cred_store(
        &self,
    ) -> Result<impl rmcp::transport::CredentialStore + 'static, TmrConnectError> {
        #[cfg(feature = "keyring")]
        {
            KeyringCredStore::new(&self.cred_user).map_err(|e| TmrConnectError::AuthError {
                msg: "Failed to initialize keyring credential store".to_string(),
                source: Some(Box::new(e)),
            })
        }
        #[cfg(not(feature = "keyring"))]
        {
            Ok(PlainCredStore::new(&self.lib_dirs, &self.cred_user))
        }
    }
}
