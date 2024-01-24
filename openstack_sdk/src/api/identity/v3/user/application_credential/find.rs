use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use tracing::trace;

use crate::api::common::CommaSeparatedList;
use crate::api::find::Findable;
use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{ApiError, Client, Pageable, Query, RestClient};

use crate::api::identity::v3::user::application_credential::{get as Get, list as List};

/// Find for user/application_credential by nameOrId.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Request<'a> {
    #[builder(setter(into), default)]
    id: Cow<'a, str>,
    #[builder(default, setter(into))]
    user_id: Cow<'a, str>,

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
        ep.user_id(self.user_id.clone());
        if let Some(headers) = &self._headers {
            ep.headers(headers.iter().map(|(k, v)| (Some(k.clone()), v.clone())));
        }
        ep.build().unwrap()
    }
    fn list_ep(&self) -> List::Request<'a> {
        let mut ep = List::Request::builder();
        ep.user_id(self.user_id.clone());
        if let Some(headers) = &self._headers {
            ep.headers(headers.iter().map(|(k, v)| (Some(k.clone()), v.clone())));
        }
        ep.name(self.id.clone());
        ep.build().unwrap()
    }
}
