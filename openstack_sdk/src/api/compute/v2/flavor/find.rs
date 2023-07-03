use derive_builder::Builder;
use tracing::{debug, info, span, trace, Level};

use itertools::Itertools;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query, RestClient};

use crate::api::compute::v2::flavor::get::Flavor as Get;
use crate::api::compute::v2::flavors::detail::get::Flavors as List;

/// Find for Flavor by NameOrId.
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

impl<'a> Findable for Flavor<'a> {
    type G = Get<'a>;
    type L = List<'a>;
    fn get_ep(&self) -> Get<'a> {
        Get::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> List<'a> {
        List::builder().build().unwrap()
    }
    /// Locate flavor in a list
    fn locate_resource_in_list<C: RestClient>(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, ApiError<C::Error>> {
        // Flavor is not supporting name as query parameter to the list.
        // Therefore it is necessary to go through complete list of flavors.
        let mut maybe_result: Option<serde_json::Value> = None;
        for item in data.iter() {
            trace!("Validate item {:?} is what we search for", item);
            if let Some(name_as_val) = item.get("name") {
                if let Some(name) = name_as_val.as_str() {
                    if name == self.id {
                        if maybe_result.is_none() {
                            maybe_result = Some(item.clone());
                        } else {
                            return Err(ApiError::IdNotUnique);
                        }
                    }
                }
            }
        }
        maybe_result.ok_or(ApiError::ResourceNotFound)
    }
}
