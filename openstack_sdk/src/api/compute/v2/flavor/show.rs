use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::trace;

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query};

/// Query for Flavor.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Flavor<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Flavor<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> FlavorBuilder<'a> {
        FlavorBuilder::default()
    }
}

impl<'a> RestEndpoint for Flavor<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("flavors/{}", self.id.as_ref()).into()
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn service_type(&self) -> Cow<'static, str> {
        "compute".into()
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavor".into())
    }
}
