//! List Ports
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use std::collections::BTreeSet;

use crate::api::Pageable;

/// Query for ports.get operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Ports<'a> {
    /// limit filter parameter
    #[builder(default, setter(into))]
    limit: Option<Cow<'a, str>>,

    /// marker filter parameter
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// binding:host_id filter parameter
    #[builder(default, setter(into))]
    binding_host_id: Option<Cow<'a, str>>,

    /// binding:profile filter parameter
    #[builder(default, setter(into))]
    binding_profile: Option<Cow<'a, str>>,

    /// binding:vif_details filter parameter
    #[builder(default, setter(into))]
    binding_vif_details: Option<Cow<'a, str>>,

    /// binding:vif_type filter parameter
    #[builder(default, setter(into))]
    binding_vif_type: Option<Cow<'a, str>>,

    /// binding:vnic_type filter parameter
    #[builder(default, setter(into))]
    binding_vnic_type: Option<Cow<'a, str>>,

    /// description filter parameter
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// device_id filter parameter
    #[builder(default, setter(into))]
    device_id: Option<Cow<'a, str>>,

    /// device_owner filter parameter
    #[builder(default, setter(into))]
    device_owner: Option<Cow<'a, str>>,

    /// fields filter parameter
    #[builder(default, private, setter(name = "_fields"))]
    fields: BTreeSet<Cow<'a, str>>,

    /// fixed_ips filter parameter
    #[builder(default, private, setter(name = "_fixed_ips"))]
    fixed_ips: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// id filter parameter
    #[builder(default, setter(into))]
    id: Option<Cow<'a, str>>,

    /// ip_address filter parameter
    #[builder(default, setter(into))]
    ip_address: Option<Cow<'a, str>>,

    /// mac_address filter parameter
    #[builder(default, setter(into))]
    mac_address: Option<Cow<'a, str>>,

    /// name filter parameter
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// network_id filter parameter
    #[builder(default, setter(into))]
    network_id: Option<Cow<'a, str>>,

    /// status filter parameter
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// subnet_id filter parameter
    #[builder(default, setter(into))]
    subnet_id: Option<Cow<'a, str>>,

    /// project_id filter parameter
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// security_groups filter parameter
    #[builder(default, setter(into))]
    security_groups: Option<Cow<'a, str>>,

    /// is_admin_state_up filter parameter
    #[builder(default)]
    is_admin_state_up: Option<bool>,

    /// is_port_security_enabled filter parameter
    #[builder(default)]
    is_port_security_enabled: Option<bool>,

    /// tags filter parameter
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// any_tags filter parameter
    #[builder(default, private, setter(name = "_any_tags"))]
    any_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_tags filter parameter
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_any_tags filter parameter
    #[builder(default, private, setter(name = "_not_any_tags"))]
    not_any_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Ports<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> PortsBuilder<'a> {
        PortsBuilder::default()
    }
}

impl<'a> PortsBuilder<'a> {
    /// fields filter parameter
    pub fn fields<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.fields
            .get_or_insert_with(BTreeSet::new)
            .extend(iter.map(Into::into));
        self
    }

    /// fixed_ips filter parameter
    pub fn fixed_ips<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.fixed_ips
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// tags filter parameter
    pub fn tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// any_tags filter parameter
    pub fn any_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.any_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not_tags filter parameter
    pub fn not_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// not_any_tags filter parameter
    pub fn not_any_tags<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_any_tags
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Ports.
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

impl<'a> RestEndpoint for Ports<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "ports".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("binding:host_id", self.binding_host_id.as_ref());
        params.push_opt("binding:profile", self.binding_profile.as_ref());
        params.push_opt("binding:vif_details", self.binding_vif_details.as_ref());
        params.push_opt("binding:vif_type", self.binding_vif_type.as_ref());
        params.push_opt("binding:vnic_type", self.binding_vnic_type.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("device_id", self.device_id.as_ref());
        params.push_opt("device_owner", self.device_owner.as_ref());
        params.extend(self.fields.iter().map(|value| ("fields", value)));
        params.push_opt("fixed_ips", self.fixed_ips.as_ref());
        params.push_opt("id", self.id.as_ref());
        params.push_opt("ip_address", self.ip_address.as_ref());
        params.push_opt("mac_address", self.mac_address.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("network_id", self.network_id.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("subnet_id", self.subnet_id.as_ref());
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("security_groups", self.security_groups.as_ref());
        params.push_opt("admin_state_up", self.is_admin_state_up);
        params.push_opt("port_security_enabled", self.is_port_security_enabled);
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.any_tags.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_any_tags.as_ref());

        params
    }

    fn service_type(&self) -> ServiceType {
        ServiceType::Network
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("ports".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Ports<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use crate::types::ServiceType;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Ports::builder().build().unwrap().service_type(),
            ServiceType::Network
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Ports::builder().build().unwrap().response_key().unwrap(),
            "ports"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path(format!("/ports",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ports": {} }));
        });

        let endpoint = Ports::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/ports",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "ports": {} }));
        });

        let endpoint = Ports::builder()
            .headers(
                [(
                    Some(HeaderName::from_static("foo")),
                    HeaderValue::from_static("bar"),
                )]
                .iter()
                .cloned(),
            )
            .header("not_foo", "not_bar")
            .build()
            .unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
