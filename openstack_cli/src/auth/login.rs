//! Perform cloud login
use anyhow::anyhow;
use async_trait::async_trait;
use clap::Args;
use std::io::{self, Write};
use tracing::info;

use crate::Cli;
use crate::{error::OpenStackCliError, OSCCommand};

use openstack_sdk::AsyncOpenStack;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct AuthArgs {
    /// Require token renewal
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub renew: bool,
}

/// login command
pub struct AuthCmd {
    pub args: AuthArgs,
}

#[async_trait]
impl OSCCommand for AuthCmd {
    async fn take_action(
        &self,
        _parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Show auth info");

        // TODO(gtema): here would be the Webbrowser based login
        // implementation

        if let Some(token) = client.get_auth_token() {
            let mut stdout = io::stdout().lock();

            stdout.write_all(&token.into_bytes())?;
            return Ok(());
        }
        Err(anyhow!("Authorization information missing").into())
    }
}
