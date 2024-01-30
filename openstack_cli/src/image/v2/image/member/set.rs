//! Sets the status for an image member.
//! *(Since Image API v2.1)*
//!
//! This call allows an image member to change his or her *member status*.
//!
//! When an image is shared with you, you have immediate access to the image.
//! What
//! updating your member status on the image does for you is that it affects
//! whether the image will appear in your image list response.
//!
//! For a more detailed discussion of image sharing, please consult [Image API
//! v2
//! Sharing](http://specs.openstack.org/openstack/glance-
//! specs/specs/api/v2/sharing-image-api-v2.html).
//!
//! Preconditions
//!
//! Synchronous Postconditions
//!
//! Normal response codes: 200
//!
//! Error response codes: 400, 401, 404, 403
//!
use async_trait::async_trait;
use clap::Args;
use serde::{Deserialize, Serialize};
use tracing::info;

use anyhow::Result;

use crate::output::OutputProcessor;
use crate::Cli;
use crate::OutputConfig;
use crate::StructTable;
use crate::{OSCCommand, OpenStackCliError};
use structable_derive::StructTable;

use openstack_sdk::AsyncOpenStack;

use crate::common::parse_json;
use crate::common::parse_key_val;
use openstack_sdk::api::image::v2::image::member::set;
use openstack_sdk::api::QueryAsync;
use serde_json::Value;

/// Command arguments
#[derive(Args, Clone, Debug)]
pub struct MemberArgs {
    /// Request Query parameters
    #[command(flatten)]
    query: QueryParameters,

    /// Path parameters
    #[command(flatten)]
    path: PathParameters,

    #[arg(long="property", value_name="key=value", value_parser=parse_key_val::<String, Value>)]
    properties: Option<Vec<(String, Value)>>,
}

/// Query parameters
#[derive(Args, Clone, Debug)]
pub struct QueryParameters {}

/// Path parameters
#[derive(Args, Clone, Debug)]
pub struct PathParameters {
    /// image_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    image_id: String,

    /// member_id parameter for /v2/images/{image_id}/members/{member_id} API
    #[arg()]
    id: String,
}

/// Member set command
pub struct MemberCmd {
    pub args: MemberArgs,
}
/// Member response representation
#[derive(Deserialize, Debug, Clone, Serialize, StructTable)]
pub struct ResponseData {
    /// The ID of the image member. An image member is usually a project (also
    /// called the “tenant”) with whom the image is shared.
    #[serde()]
    #[structable(optional)]
    member_id: Option<String>,

    /// The UUID of the image.
    #[serde()]
    #[structable(optional)]
    image_id: Option<String>,

    /// The date and time when the resource was created.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC.
    #[serde()]
    #[structable(optional)]
    created_at: Option<String>,

    /// The date and time when the resource was updated.
    ///
    ///
    /// The date and time stamp format is [ISO
    /// 8601](https://en.wikipedia.org/wiki/ISO_8601):
    ///
    ///
    ///
    /// ```text
    /// CCYY-MM-DDThh:mm:ss±hh:mm
    ///
    /// ```
    ///
    ///
    /// For example, `2015-08-27T09:49:58-05:00`.
    ///
    ///
    /// The `±hh:mm` value, if included, is the time zone as an offset
    /// from UTC. In the previous example, the offset value is `-05:00`.
    ///
    ///
    /// If the `updated\_at` date and time stamp is not set, its value is
    /// `null`.
    #[serde()]
    #[structable(optional)]
    updated_at: Option<String>,

    /// The status of this image member. Value is one of `pending`,
    /// `accepted`, `rejected`.
    #[serde()]
    #[structable(optional)]
    status: Option<String>,

    /// The URL for the schema describing an image member.
    #[serde()]
    #[structable(optional)]
    schema: Option<String>,
}

#[async_trait]
impl OSCCommand for MemberCmd {
    async fn take_action(
        &self,
        parsed_args: &Cli,
        client: &mut AsyncOpenStack,
    ) -> Result<(), OpenStackCliError> {
        info!("Set Member with {:?}", self.args);

        let op = OutputProcessor::from_args(parsed_args);
        op.validate_args(parsed_args)?;
        info!("Parsed args: {:?}", self.args);

        let mut ep_builder = set::Request::builder();

        // Set path parameters
        ep_builder.image_id(&self.args.path.image_id);
        ep_builder.id(&self.args.path.id);
        // Set query parameters
        // Set body parameters
        if let Some(properties) = &self.args.properties {
            ep_builder.properties(properties.iter().cloned());
        }

        let ep = ep_builder
            .build()
            .map_err(|x| OpenStackCliError::EndpointBuild(x.to_string()))?;

        let data = ep.query_async(client).await?;
        op.output_single::<ResponseData>(data)?;
        Ok(())
    }
}
