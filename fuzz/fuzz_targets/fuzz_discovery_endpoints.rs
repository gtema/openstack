#![no_main]

use arbitrary::Arbitrary;
use bytes::Bytes;
use libfuzzer_sys::fuzz_target;
use serde_json::{Map, Value};
use url::Url;

extern crate openstack_sdk;

// `extract_discovery_endpoints` tries three different JSON shapes in sequence
// for a version-discovery document (`{"versions": [...]}`, `{"version": {}}`,
// Keystone's `{"versions": {"values": [...]}}`), then walks each version's
// `links` looking for a `rel: "self"` entry and resolves it via `expand_link`.
// We combine two Arbitrary-derived generation modes in one target: fully raw
// bytes (to check robustness against garbage input) and a structured model of
// the three document shapes (to actually reach the per-version/link handling
// that raw bytes essentially never survive JSON parsing long enough to hit).

#[derive(Debug, Arbitrary)]
struct FuzzLink {
    href: String,
    rel: FuzzRel,
}

#[derive(Debug, Arbitrary)]
enum FuzzRel {
    SelfRel,
    Other(String),
}

impl FuzzLink {
    fn to_value(&self) -> Value {
        let rel = match &self.rel {
            FuzzRel::SelfRel => "self".to_string(),
            FuzzRel::Other(s) => s.clone(),
        };
        let mut obj = Map::new();
        obj.insert("href".into(), Value::String(self.href.clone()));
        obj.insert("rel".into(), Value::String(rel));
        Value::Object(obj)
    }
}

#[derive(Debug, Arbitrary)]
struct FuzzEndpointVersion {
    id: String,
    status: String,
    version: Option<String>,
    min_version: Option<String>,
    max_version: Option<String>,
    links: Vec<FuzzLink>,
}

impl FuzzEndpointVersion {
    fn to_value(&self) -> Value {
        let mut obj = Map::new();
        obj.insert("id".into(), Value::String(self.id.clone()));
        obj.insert("status".into(), Value::String(self.status.clone()));
        if let Some(v) = &self.version {
            obj.insert("version".into(), Value::String(v.clone()));
        }
        if let Some(v) = &self.min_version {
            obj.insert("min_version".into(), Value::String(v.clone()));
        }
        if let Some(v) = &self.max_version {
            obj.insert("max_version".into(), Value::String(v.clone()));
        }
        let links: Vec<Value> = self.links.iter().map(FuzzLink::to_value).collect();
        obj.insert("links".into(), Value::Array(links));
        Value::Object(obj)
    }
}

#[derive(Debug, Arbitrary)]
enum FuzzBody {
    /// `{"versions": [...]}` (Nova/Glance-style unversioned discovery doc).
    Versions(Vec<FuzzEndpointVersion>),
    /// `{"version": {...}}` (versioned endpoint discovery doc).
    Version(FuzzEndpointVersion),
    /// `{"versions": {"values": [...]}}` (Keystone-style).
    VersionsValues(Vec<FuzzEndpointVersion>),
}

impl FuzzBody {
    fn to_value(&self) -> Value {
        let mut obj = Map::new();
        match self {
            FuzzBody::Versions(versions) => {
                let versions: Vec<Value> =
                    versions.iter().map(FuzzEndpointVersion::to_value).collect();
                obj.insert("versions".into(), Value::Array(versions));
            }
            FuzzBody::Version(version) => {
                obj.insert("version".into(), version.to_value());
            }
            FuzzBody::VersionsValues(versions) => {
                let values: Vec<Value> =
                    versions.iter().map(FuzzEndpointVersion::to_value).collect();
                let mut inner = Map::new();
                inner.insert("values".into(), Value::Array(values));
                obj.insert("versions".into(), Value::Object(inner));
            }
        }
        Value::Object(obj)
    }
}

#[derive(Debug, Arbitrary)]
enum FuzzInput {
    Structured(FuzzBody),
    RawBytes(Vec<u8>),
}

#[derive(Debug, Arbitrary)]
enum FuzzHost {
    FooBar,
    ExampleOrg,
    Localhost,
}

impl FuzzHost {
    fn as_str(&self) -> &'static str {
        match self {
            FuzzHost::FooBar => "foo.bar",
            FuzzHost::ExampleOrg => "example.org",
            FuzzHost::Localhost => "localhost",
        }
    }
}

#[derive(Debug, Arbitrary)]
struct Input {
    body: FuzzInput,
    discovery_host: FuzzHost,
    service_type: String,
}

fuzz_target!(|input: Input| {
    let Ok(discovery_url) = Url::parse(&format!("http://{}/", input.discovery_host.as_str()))
    else {
        return;
    };

    let data: Vec<u8> = match &input.body {
        FuzzInput::Structured(body) => body.to_value().to_string().into_bytes(),
        FuzzInput::RawBytes(bytes) => bytes.clone(),
    };

    let _ = openstack_sdk::catalog::extract_discovery_endpoints(
        &discovery_url,
        &Bytes::from(data),
        &input.service_type,
    );
});
