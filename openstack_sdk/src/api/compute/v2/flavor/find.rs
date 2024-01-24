use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use tracing::trace;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query, RestClient};

use crate::api::compute::v2::flavor::{get as Get, list_detailed as List};

/// Find for flavor by nameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Request<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> RequestBuilder<'a> {
        RequestBuilder::default()
    }
}

impl<'a> RequestBuilder<'a> {
    /// Add a single header to the Volume.
    pub fn header(&mut self, header_name: &'static str, header_value: &'static str) -> &mut Self
where {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .insert(header_name, HeaderValue::from_static(header_value));
        self
    }

    /// Add multiple headers.
    pub fn headers<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<(Option<HeaderName>, HeaderValue)>,
    {
        self._headers
            .get_or_insert(None)
            .get_or_insert_with(HeaderMap::new)
            .extend(iter.map(Into::into));
        self
    }
}

impl<'a> Findable for Request<'a> {
    type G = Get::Request<'a>;
    type L = List::Request<'a>;
    fn get_ep(&self) -> Get::Request<'a> {
        let mut ep = Get::Request::builder();
        ep.id(self.id.clone());
        if let Some(headers) = &self._headers {
            ep.headers(headers.iter().map(|(k, v)| (Some(k.clone()), v.clone())));
        }
        ep.build().unwrap()
    }
    fn list_ep(&self) -> List::Request<'a> {
        let mut ep = List::Request::builder();
        if let Some(headers) = &self._headers {
            ep.headers(headers.iter().map(|(k, v)| (Some(k.clone()), v.clone())));
        }
        ep.build().unwrap()
    }
    /// Locate flavor in a list
    fn locate_resource_in_list<C: RestClient>(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, ApiError<C::Error>> {
        // flavor is not supporting name as query parameter to the list.
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
