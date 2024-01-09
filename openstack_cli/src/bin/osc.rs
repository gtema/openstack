//! OpenStack CLI
//!
//! The binary of the CLI
#![deny(missing_docs)]

use openstack_cli::error::OpenStackCliError;

#[tokio::main]
async fn main() -> Result<(), OpenStackCliError> {
    match openstack_cli::entry_point().await {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{}", e);
            return Err(e);
        }
    }
}
