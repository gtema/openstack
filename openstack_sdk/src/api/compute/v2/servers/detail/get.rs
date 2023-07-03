//! List detailed Servers
use derive_builder::Builder;
use http::{HeaderMap, HeaderName, HeaderValue};
use std::collections::BTreeSet;

use crate::api::common::CommaSeparatedList;
use crate::api::rest_endpoint_prelude::*;

use crate::api::Pageable;

/// Query for servers.get_detailed operation.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Servers<'a> {
    /// limit filter parameter
    #[builder(default, setter(into))]
    limit: Option<Cow<'a, str>>,

    /// marker filter parameter
    #[builder(default, setter(into))]
    marker: Option<Cow<'a, str>>,

    /// auto_disk_config filter parameter
    #[builder(default, setter(into))]
    auto_disk_config: Option<Cow<'a, str>>,

    /// availability_zone filter parameter
    #[builder(default, setter(into))]
    availability_zone: Option<Cow<'a, str>>,

    /// created_at filter parameter
    #[builder(default, setter(into))]
    created_at: Option<Cow<'a, str>>,

    /// description filter parameter
    #[builder(default, setter(into))]
    description: Option<Cow<'a, str>>,

    /// flavor filter parameter
    #[builder(default, setter(into))]
    flavor: Option<Cow<'a, str>>,

    /// hostname filter parameter
    #[builder(default, setter(into))]
    hostname: Option<Cow<'a, str>>,

    /// image filter parameter
    #[builder(default, setter(into))]
    image: Option<Cow<'a, str>>,

    /// kernel_id filter parameter
    #[builder(default, setter(into))]
    kernel_id: Option<Cow<'a, str>>,

    /// key_name filter parameter
    #[builder(default, setter(into))]
    key_name: Option<Cow<'a, str>>,

    /// launch_index filter parameter
    #[builder(default)]
    launch_index: Option<u32>,

    /// launched_at filter parameter
    #[builder(default, setter(into))]
    launched_at: Option<Cow<'a, str>>,

    /// locked_by filter parameter
    #[builder(default, setter(into))]
    locked_by: Option<Cow<'a, str>>,

    /// name filter parameter
    #[builder(default, setter(into))]
    name: Option<Cow<'a, str>>,

    /// node filter parameter
    #[builder(default, setter(into))]
    node: Option<Cow<'a, str>>,

    /// power_state filter parameter
    #[builder(default, setter(into))]
    power_state: Option<Cow<'a, str>>,

    /// progress filter parameter
    #[builder(default)]
    progress: Option<u32>,

    /// project_id filter parameter
    #[builder(default, setter(into))]
    project_id: Option<Cow<'a, str>>,

    /// ramdisk_id filter parameter
    #[builder(default, setter(into))]
    ramdisk_id: Option<Cow<'a, str>>,

    /// reservation_id filter parameter
    #[builder(default, setter(into))]
    reservation_id: Option<Cow<'a, str>>,

    /// root_device_name filter parameter
    #[builder(default, setter(into))]
    root_device_name: Option<Cow<'a, str>>,

    /// status filter parameter
    #[builder(default, setter(into))]
    status: Option<Cow<'a, str>>,

    /// task_state filter parameter
    #[builder(default, setter(into))]
    task_state: Option<Cow<'a, str>>,

    /// terminated_at filter parameter
    #[builder(default, setter(into))]
    terminated_at: Option<Cow<'a, str>>,

    /// user_id filter parameter
    #[builder(default, setter(into))]
    user_id: Option<Cow<'a, str>>,

    /// vm_state filter parameter
    #[builder(default, setter(into))]
    vm_state: Option<Cow<'a, str>>,

    /// sort_key filter parameter
    #[builder(default, setter(into))]
    sort_key: Option<Cow<'a, str>>,

    /// sort_dir filter parameter
    #[builder(default, setter(into))]
    sort_dir: Option<Cow<'a, str>>,

    /// access_ipv4 filter parameter
    #[builder(default, setter(into))]
    access_ip_v4: Option<Cow<'a, str>>,

    /// access_ipv6 filter parameter
    #[builder(default, setter(into))]
    access_ip_v6: Option<Cow<'a, str>>,

    /// has_config_drive filter parameter
    #[builder(default, setter(into))]
    config_drive: Option<Cow<'a, str>>,

    /// deleted_only filter parameter
    #[builder(default, setter(into))]
    deleted: Option<Cow<'a, str>>,

    /// compute_host filter parameter
    #[builder(default, setter(into))]
    host: Option<Cow<'a, str>>,

    /// is_soft_deleted filter parameter
    #[builder(default, setter(into))]
    soft_deleted: Option<Cow<'a, str>>,

    /// ipv4_address filter parameter
    #[builder(default, setter(into))]
    ip: Option<Cow<'a, str>>,

    /// ipv6_address filter parameter
    #[builder(default, setter(into))]
    ip6: Option<Cow<'a, str>>,

    /// changes_since filter parameter
    #[builder(default, setter(into))]
    changes_since: Option<Cow<'a, str>>,

    /// changes_before filter parameter
    #[builder(default, setter(into))]
    changes_before: Option<Cow<'a, str>>,

    /// id filter parameter
    #[builder(default, setter(into))]
    uuid: Option<Cow<'a, str>>,

    /// all_projects filter parameter
    #[builder(default, setter(into))]
    all_tenants: Option<Cow<'a, str>>,

    /// tags filter parameter
    #[builder(default, private, setter(name = "_tags"))]
    tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// any_tags filter parameter
    #[builder(default, private, setter(name = "_tags_any"))]
    tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_tags filter parameter
    #[builder(default, private, setter(name = "_not_tags"))]
    not_tags: Option<CommaSeparatedList<Cow<'a, str>>>,

    /// not_any_tags filter parameter
    #[builder(default, private, setter(name = "_not_tags_any"))]
    not_tags_any: Option<CommaSeparatedList<Cow<'a, str>>>,

    #[builder(setter(name = "_headers"), default, private)]
    _headers: Option<HeaderMap>,
}

impl<'a> Servers<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> ServersBuilder<'a> {
        ServersBuilder::default()
    }
}

impl<'a> ServersBuilder<'a> {
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
    pub fn tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.tags_any
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
    pub fn not_tags_any<I, T>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = T>,
        T: Into<Cow<'a, str>>,
    {
        self.not_tags_any
            .get_or_insert(None)
            .get_or_insert_with(CommaSeparatedList::new)
            .extend(iter.map(Into::into));
        self
    }

    /// Add a single header to the Servers.
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

impl<'a> RestEndpoint for Servers<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "servers/detail".to_string().into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();
        params.push_opt("limit", self.limit.as_ref());
        params.push_opt("marker", self.marker.as_ref());
        params.push_opt("auto_disk_config", self.auto_disk_config.as_ref());
        params.push_opt("availability_zone", self.availability_zone.as_ref());
        params.push_opt("created_at", self.created_at.as_ref());
        params.push_opt("description", self.description.as_ref());
        params.push_opt("flavor", self.flavor.as_ref());
        params.push_opt("hostname", self.hostname.as_ref());
        params.push_opt("image", self.image.as_ref());
        params.push_opt("kernel_id", self.kernel_id.as_ref());
        params.push_opt("key_name", self.key_name.as_ref());
        params.push_opt("launch_index", self.launch_index);
        params.push_opt("launched_at", self.launched_at.as_ref());
        params.push_opt("locked_by", self.locked_by.as_ref());
        params.push_opt("name", self.name.as_ref());
        params.push_opt("node", self.node.as_ref());
        params.push_opt("power_state", self.power_state.as_ref());
        params.push_opt("progress", self.progress);
        params.push_opt("project_id", self.project_id.as_ref());
        params.push_opt("ramdisk_id", self.ramdisk_id.as_ref());
        params.push_opt("reservation_id", self.reservation_id.as_ref());
        params.push_opt("root_device_name", self.root_device_name.as_ref());
        params.push_opt("status", self.status.as_ref());
        params.push_opt("task_state", self.task_state.as_ref());
        params.push_opt("terminated_at", self.terminated_at.as_ref());
        params.push_opt("user_id", self.user_id.as_ref());
        params.push_opt("vm_state", self.vm_state.as_ref());
        params.push_opt("sort_key", self.sort_key.as_ref());
        params.push_opt("sort_dir", self.sort_dir.as_ref());
        params.push_opt("access_ip_v4", self.access_ip_v4.as_ref());
        params.push_opt("access_ip_v6", self.access_ip_v6.as_ref());
        params.push_opt("config_drive", self.config_drive.as_ref());
        params.push_opt("deleted", self.deleted.as_ref());
        params.push_opt("host", self.host.as_ref());
        params.push_opt("soft_deleted", self.soft_deleted.as_ref());
        params.push_opt("ip", self.ip.as_ref());
        params.push_opt("ip6", self.ip6.as_ref());
        params.push_opt("changes-since", self.changes_since.as_ref());
        params.push_opt("changes-before", self.changes_before.as_ref());
        params.push_opt("uuid", self.uuid.as_ref());
        params.push_opt("all_tenants", self.all_tenants.as_ref());
        params.push_opt("tags", self.tags.as_ref());
        params.push_opt("tags-any", self.tags_any.as_ref());
        params.push_opt("not-tags", self.not_tags.as_ref());
        params.push_opt("not-tags-any", self.not_tags_any.as_ref());

        params
    }

    fn service_type(&self) -> Cow<'static, str> {
        "compute".into()
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("servers".into())
    }

    /// Returns headers to be set into the request
    fn request_headers(&self) -> Option<&HeaderMap> {
        self._headers.as_ref()
    }
}
impl<'a> Pageable for Servers<'a> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{self, Query, RawQuery};
    use crate::test::client::MockServerClient;
    use http::{HeaderName, HeaderValue};
    use serde::Deserialize;
    use serde_json::json;

    #[test]
    fn test_service_type() {
        assert_eq!(
            Servers::builder().build().unwrap().service_type(),
            "compute"
        );
    }

    #[test]
    fn test_response_key() {
        assert_eq!(
            Servers::builder().build().unwrap().response_key().unwrap(),
            "servers"
        );
    }

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/servers/detail",));

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "servers": {} }));
        });

        let endpoint = Servers::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }

    #[test]
    fn endpoint_headers() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path(format!("/servers/detail",))
                .header("foo", "bar")
                .header("not_foo", "not_bar");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "servers": {} }));
        });

        let endpoint = Servers::builder()
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
