use derive_builder::Builder;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query, RestClient};

use crate::api::block_storage::v3::volume::get::Volume as Get;
use crate::api::block_storage::v3::volumes::detail::get::Volumes as List;

/// Find for Volume by NameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Volume<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
    #[builder(setter(into), default)]
    project_id: Cow<'a, str>,
}

impl<'a> Volume<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> VolumeBuilder<'a> {
        VolumeBuilder::default()
    }
}

impl<'a> Findable for Volume<'a> {
    type G = Get<'a>;
    type L = List<'a>;
    fn get_ep(&self) -> Get<'a> {
        Get::builder()
            .id(self.id.clone())
            .project_id(self.project_id.clone())
            .build()
            .unwrap()
    }
    fn list_ep(&self) -> List<'a> {
        List::builder()
            .project_id(self.project_id.clone())
            .name(self.id.clone())
            .build()
            .unwrap()
    }
}
