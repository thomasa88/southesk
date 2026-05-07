// Copyright 2026 Thomas Axelsson
// SPDX-License-Identifier: MIT

use rmcp::{RoleClient, model::InitializeRequestParams, service::RunningService};

mod connection;
mod tools;

/// The Montrose MCP client
///
/// The client must first be connected, then the Montrose API functions can be
/// used.
///
/// [`TmrClient<Connected>`] provides the available API functions.
///
/// The user will automatically be requested to authenticate if there is no
/// valid cached OAuth token. Use [`TmrClient::connect_with_cb()`] to customize
/// how the user is requested to authenticate.
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
    state: S,
}

pub trait State {}

pub struct Disconnected {}
pub struct Connected {
    client: RunningService<RoleClient, InitializeRequestParams>,
}

impl State for Disconnected {}
impl State for Connected {}

impl<S: State> TmrClient<S> {}
