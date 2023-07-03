use derive_builder::Builder;
use serde::de::DeserializeOwned;

use crate::api::rest_endpoint_prelude::*;
use crate::api::ParamValue;

use crate::api::{Client, Pageable};

/// Query for flavors.
#[derive(Debug, Builder, Clone)]
#[builder(setter(strip_option))]
pub struct Flavors<'a> {
    #[builder(setter(into), default)]
    pub name: Option<Cow<'a, str>>,
    #[builder(default)]
    pub is_public: Option<bool>,
}

impl<'a> Flavors<'a> {
    /// Create a builder for the endpoint.
    pub fn builder() -> FlavorsBuilder<'a> {
        FlavorsBuilder::default()
    }
}

impl<'a> RestEndpoint for Flavors<'a> {
    fn method(&self) -> Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        "flavors/detail".into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params.push_opt("name", self.name.as_ref());
        params.push_opt("is_public", self.is_public);

        params
    }

    fn service_type(&self) -> Cow<'static, str> {
        "compute".into()
    }

    fn response_key(&self) -> Option<Cow<'static, str>> {
        Some("flavors".into())
    }
}

impl<'a> Pageable for Flavors<'a> {}

#[cfg(test)]
mod tests {
    use crate::api::compute::v2::flavor::list::Flavors;
    use crate::api::{self, Query};
    use crate::test::client::MockServerClient;
    use serde_json::json;

    #[test]
    fn endpoint() {
        let client = MockServerClient::new();
        let mock = client.server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/flavors/detail");

            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "dummy": {} }));
        });

        let endpoint = Flavors::builder().build().unwrap();
        let _: serde_json::Value = endpoint.query(&client).unwrap();
        mock.assert();
    }
}
