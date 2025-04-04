// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use serde::de::DeserializeOwned;

use crate::api::{ApiError, RestClient, RestEndpoint};

#[cfg(feature = "async")]
use crate::api::{AsyncClient, QueryAsync};
#[cfg(feature = "sync")]
use crate::api::{Client, Query};

/// Trait for findable resources that combines GET and LIST endpoint
pub trait Findable {
    /// GET endpoint type
    type G;
    /// LIST endpoint type
    type L;
    /// return GET RestEndpoint with query parameters set
    fn get_ep(&self) -> Self::G;
    /// return LIST RestEndpoint with query parameters set
    fn list_ep(&self) -> Self::L;
    /// Locate single resource in resources list
    fn locate_resource_in_list<C: RestClient>(
        &self,
        data: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, ApiError<C::Error>> {
        // Nearly all resources support name as a query parameter to the
        // list. Therefore it is ok to just get there is a singe entry.
        // There are, however, exceptions like compute.flavor. In this case
        // this method can be overridden to go through list of received
        // elements and compare entries one by one.
        match data.len() {
            0 => Err(ApiError::ResourceNotFound),
            1 => Ok(data[0].clone()),
            2.. => Err(ApiError::IdNotUnique),
        }
    }
}

/// Resource search mode
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum FindMode {
    /// Search by name or id.
    #[default]
    NameOrId,
    /// Search by name.
    Name,
}

/// A query modifier that wraps Findable resources to workaround conflicting trait implementations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Find<E> {
    /// Findable endpoint
    pub(in crate::api::find) findable: E,
    /// Find mode
    mode: FindMode,
}

/// Function wrapper for findable resources to locate resource by name or id
pub fn find<F>(findable: F) -> Find<F> {
    Find {
        findable,
        mode: FindMode::NameOrId,
    }
}

/// Function wrapper for findable resources to locate by name
pub fn find_by_name<F>(findable: F) -> Find<F> {
    Find {
        findable,
        mode: FindMode::Name,
    }
}

#[cfg(feature = "sync")]
impl<E, T, C> Query<T, C> for Find<E>
where
    E: Findable,
    <E as Findable>::G: RestEndpoint,
    <E as Findable>::L: RestEndpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let res = match self.mode {
            FindMode::NameOrId => {
                let get_ep = self.findable.get_ep();
                let get_res = get_ep.query(client);
                match get_res {
                    Err(x) => match x {
                        crate::api::ApiError::ResourceNotFound
                        | crate::api::ApiError::OpenStack {
                            status: http::StatusCode::NOT_FOUND,
                            ..
                        }
                        | crate::api::ApiError::OpenStack {
                            // Some services return 400 when "ID" is not ID like
                            status: http::StatusCode::BAD_REQUEST,
                            ..
                        }
                        | crate::api::ApiError::OpenStackService {
                            status: http::StatusCode::NOT_FOUND,
                            ..
                        }
                        | crate::api::ApiError::OpenStackService {
                            status: http::StatusCode::BAD_REQUEST,
                            ..
                        }
                        | crate::api::ApiError::OpenStackUnrecognized {
                            status: http::StatusCode::NOT_FOUND,
                            ..
                        }
                        | crate::api::ApiError::OpenStackUnrecognized {
                            status: http::StatusCode::BAD_REQUEST,
                            ..
                        } => {
                            let list_ep = self.findable.list_ep();
                            let data: Vec<serde_json::Value> = list_ep.query(client)?;
                            self.findable.locate_resource_in_list::<C>(data)?
                        }
                        _ => {
                            return Err(x);
                        }
                    },
                    Ok(x) => x,
                }
            }
            FindMode::Name => {
                let list_ep = self.findable.list_ep();
                let data: Vec<serde_json::Value> = list_ep.query(client)?;
                self.findable.locate_resource_in_list::<C>(data)?
            }
        };

        match serde_json::from_value::<T>(res) {
            Ok(r) => Ok(r),
            Err(e) => Err(ApiError::data_type::<T>(e)),
        }
    }
}

#[cfg(feature = "async")]
#[async_trait]
impl<E, T, C> QueryAsync<T, C> for Find<E>
where
    E: Findable + Sync,
    <E as Findable>::G: RestEndpoint + Sync + Send,
    <E as Findable>::L: RestEndpoint + Sync + Send,
    T: DeserializeOwned + Send + 'static,
    C: AsyncClient + Sync,
{
    async fn query_async(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let res = match self.mode {
            FindMode::NameOrId => {
                let get_ep = self.findable.get_ep();
                let get_res = get_ep.query_async(client).await;
                match get_res {
                    Err(x) => match x {
                        crate::api::ApiError::ResourceNotFound
                        | crate::api::ApiError::OpenStack {
                            status: http::StatusCode::NOT_FOUND,
                            ..
                        }
                        | crate::api::ApiError::OpenStack {
                            status: http::StatusCode::BAD_REQUEST,
                            ..
                        }
                        | crate::api::ApiError::OpenStackService {
                            status: http::StatusCode::NOT_FOUND,
                            ..
                        }
                        | crate::api::ApiError::OpenStackService {
                            status: http::StatusCode::BAD_REQUEST,
                            ..
                        }
                        | crate::api::ApiError::OpenStackUnrecognized {
                            status: http::StatusCode::NOT_FOUND,
                            ..
                        }
                        | crate::api::ApiError::OpenStackUnrecognized {
                            status: http::StatusCode::BAD_REQUEST,
                            ..
                        } => {
                            let list_ep = self.findable.list_ep();
                            let data: Vec<serde_json::Value> = list_ep.query_async(client).await?;
                            self.findable.locate_resource_in_list::<C>(data)?
                        }
                        _ => {
                            return Err(x);
                        }
                    },
                    Ok(x) => x,
                }
            }
            FindMode::Name => {
                let list_ep = self.findable.list_ep();
                let data: Vec<serde_json::Value> = list_ep.query_async(client).await?;
                self.findable.locate_resource_in_list::<C>(data)?
            }
        };

        match serde_json::from_value::<T>(res) {
            Ok(r) => Ok(r),
            Err(e) => Err(ApiError::data_type::<T>(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use httpmock::MockServer;
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[cfg(feature = "sync")]
    use crate::api::Query;
    #[cfg(feature = "async")]
    use crate::api::QueryAsync;
    use crate::api::find::Findable;
    use crate::api::rest_endpoint_prelude::*;
    use crate::api::{self, ApiError};
    use crate::test::client::FakeOpenStackClient;
    use derive_builder::Builder;

    #[derive(Debug, Builder, Clone)]
    struct GetDummy<'a> {
        #[builder(setter(into), default)]
        id: Cow<'a, str>,
    }

    impl<'a> GetDummy<'a> {
        pub fn builder() -> GetDummyBuilder<'a> {
            GetDummyBuilder::default()
        }
    }

    impl RestEndpoint for GetDummy<'_> {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            format!("dummies/{}", self.id.as_ref()).into()
        }
        fn service_type(&self) -> ServiceType {
            ServiceType::from("dummy")
        }
        fn response_key(&self) -> Option<Cow<'static, str>> {
            Some("resource".into())
        }
    }

    #[derive(Debug, Builder, Clone)]
    struct ListDummies<'a> {
        #[builder(setter(into), default)]
        name: Option<Cow<'a, str>>,
    }

    impl<'a> ListDummies<'a> {
        pub fn builder() -> ListDummiesBuilder<'a> {
            ListDummiesBuilder::default()
        }
    }

    impl RestEndpoint for ListDummies<'_> {
        fn method(&self) -> http::Method {
            http::Method::GET
        }

        fn endpoint(&self) -> Cow<'static, str> {
            "dummies".into()
        }
        fn service_type(&self) -> ServiceType {
            ServiceType::from("dummy")
        }
        fn response_key(&self) -> Option<Cow<'static, str>> {
            Some("resources".into())
        }
        fn parameters(&self) -> QueryParams {
            let mut params = QueryParams::default();

            params.push_opt("name", self.name.as_ref());

            params
        }
    }

    struct Dummy<'a> {
        id: Cow<'a, str>,
    }

    impl<'a> Findable for Dummy<'a> {
        type G = GetDummy<'a>;
        type L = ListDummies<'a>;
        fn get_ep(&self) -> GetDummy<'a> {
            GetDummy::builder().id(self.id.clone()).build().unwrap()
        }
        fn list_ep(&self) -> ListDummies<'a> {
            ListDummies::builder()
                .name(self.id.clone())
                .build()
                .unwrap()
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct DummyResult {
        id: String,
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_get_1() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resource": {"id": "abc"} }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query(&client);
        if let Ok(x) = res {
            assert_eq!("abc", x.id);
            get_mock.assert();
        } else {
            panic!("unexpected response");
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_get_1_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resource": {"id": "abc"} }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query_async(&client).await;
        if let Ok(x) = res {
            assert_eq!("abc", x.id);
            get_mock.assert();
        } else {
            panic!("unexpected response");
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_get_0_list_1() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query(&client);
        get_mock.assert();
        list_mock.assert();
        let _err = res.unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_get_0_list_1_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query_async(&client).await;
        get_mock.assert();
        list_mock.assert();
        let _err = res.unwrap();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_get_0_400_list_1() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(400);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query(&client);
        get_mock.assert();
        list_mock.assert();
        let _err = res.unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_get_0_400_list_1_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(400);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query_async(&client).await;
        get_mock.assert();
        list_mock.assert();
        let _err = res.unwrap();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_by_name_0_list_1() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find_by_name(ep).query(&client);
        list_mock.assert();
        let _err = res.unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_by_name_list_1_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find_by_name(ep).query_async(&client).await;
        list_mock.assert();
        let _err = res.unwrap();
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_get_0_list_2() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}, {"id": "abc2"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query(&client);
        get_mock.assert();
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::IdNotUnique) {
            panic!("Unexpected error");
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_get_0_list_2_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}, {"id": "abc2"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query_async(&client).await;
        get_mock.assert();
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::IdNotUnique) {
            panic!("Unexpected error");
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_by_name_list_2() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}, {"id": "abc2"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find_by_name(ep).query(&client);
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::IdNotUnique) {
            panic!("Unexpected error");
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_by_name_list_2_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [{"id": "abc"}, {"id": "abc2"}] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find_by_name(ep).query_async(&client).await;
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::IdNotUnique) {
            panic!("Unexpected error");
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_get_0_list_0() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query(&client);
        get_mock.assert();
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::ResourceNotFound) {
            panic!("Unexpected error: {}", err);
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_get_0_list_0_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let get_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET).path("/dummies/abc");
            then.status(404);
        });
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find(ep).query_async(&client).await;
        get_mock.assert();
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::ResourceNotFound) {
            panic!("Unexpected error: {}", err);
        }
    }

    #[cfg(feature = "sync")]
    #[test]
    fn test_by_name_list_0() {
        let server = MockServer::start();
        let client = FakeOpenStackClient::new(server.base_url());
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find_by_name(ep).query(&client);
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::ResourceNotFound) {
            panic!("Unexpected error: {}", err);
        }
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_by_name_list_0_async() {
        let server = MockServer::start_async().await;
        let client = FakeOpenStackClient::new(server.base_url());
        let list_mock = server.mock(|when, then| {
            when.method(httpmock::Method::GET)
                .path("/dummies")
                .query_param("name", "abc");
            then.status(200)
                .header("content-type", "application/json")
                .json_body(json!({ "resources": [] }));
        });
        let ep = Dummy { id: "abc".into() };
        let res: Result<DummyResult, _> = api::find_by_name(ep).query_async(&client).await;
        list_mock.assert();
        let err = res.unwrap_err();
        if !matches!(err, ApiError::ResourceNotFound) {
            panic!("Unexpected error: {}", err);
        }
    }
}
