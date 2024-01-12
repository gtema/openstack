use derive_builder::Builder;
use serde::de::DeserializeOwned;
use tracing::trace;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query, RestClient};

use crate::api::compute::v2::os_keypair::{get as Get, list as List};

/// Find for os_keypair by nameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
}

impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> Findable for Request<'a> {
    type G = Get::Request<'a>;
    type L = List::Request<'a>;
    fn get_ep(&self) -> Get::Request<'a> {
        Get::Request::builder().id(self.id.clone()).build().unwrap()
    }
    fn list_ep(&self) -> List::Request<'a> {
        List::Request::builder().build().unwrap()
    }
    /// Locate os_keypair in a list
    fn locate_resource_in_list<C: RestClient>(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, ApiError<C::Error>> {
        // os_keypair is not supporting name as query parameter to the list.
        // Therefore it is necessary to go through complete list of results.
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
