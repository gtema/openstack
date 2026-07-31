#![no_main]

use std::borrow::Cow;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use serde_json::{Map, Value};
use url::Url;

extern crate openstack_sdk;

// `next_page_from_body` walks an already-parsed `serde_json::Value` looking
// for OpenStack pagination hints ("links", "<resource>_links", "next").
// Random bytes almost never parse as JSON at all, so instead of fuzzing raw
// bytes we derive `Arbitrary` on a small model of the shapes the function
// actually branches on, and build the `serde_json::Value` from that. This
// lets libFuzzer's mutations explore the interesting branches (missing
// "rel", relative vs. absolute "href"/"next", matching vs. differing host)
// far more effectively than byte-level fuzzing would.

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
enum FuzzTargetUrl {
    Relative(String),
    AbsoluteSameHost(String),
    AbsoluteOtherHost(FuzzHost, String),
    Raw(String),
}

impl FuzzTargetUrl {
    fn render(&self, own_host: &FuzzHost) -> String {
        match self {
            FuzzTargetUrl::Relative(path) => format!("/{path}"),
            FuzzTargetUrl::AbsoluteSameHost(path) => format!("http://{}/{path}", own_host.as_str()),
            FuzzTargetUrl::AbsoluteOtherHost(host, path) => {
                format!("http://{}/{path}", host.as_str())
            }
            FuzzTargetUrl::Raw(s) => s.clone(),
        }
    }
}

#[derive(Debug, Arbitrary)]
struct FuzzLinkEntry {
    rel: Option<String>,
    href: Option<FuzzTargetUrl>,
}

impl FuzzLinkEntry {
    fn to_value(&self, own_host: &FuzzHost) -> Value {
        let mut obj = Map::new();
        if let Some(rel) = &self.rel {
            obj.insert("rel".into(), Value::String(rel.clone()));
        }
        if let Some(href) = &self.href {
            obj.insert("href".into(), Value::String(href.render(own_host)));
        }
        Value::Object(obj)
    }
}

#[derive(Debug, Arbitrary)]
enum FuzzBody {
    Empty,
    LinksArray(Vec<FuzzLinkEntry>),
    ResourceLinksArray(Vec<FuzzLinkEntry>),
    LinksDict { next: Option<String> },
    NextField(Option<FuzzTargetUrl>),
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    body: FuzzBody,
    response_key: Option<String>,
    base_host: FuzzHost,
    base_port: Option<u16>,
}

fuzz_target!(|input: FuzzInput| {
    let base_url_str = match input.base_port {
        Some(port) => format!("http://{}:{port}/v1", input.base_host.as_str()),
        None => format!("http://{}/v1", input.base_host.as_str()),
    };
    let Ok(base_endpoint) = Url::parse(&base_url_str) else {
        return;
    };

    let response_key: Option<Cow<'_, str>> = input.response_key.clone().map(Cow::Owned);

    let content = match &input.body {
        FuzzBody::Empty => Value::Object(Map::new()),
        FuzzBody::LinksArray(entries) => {
            let links: Vec<Value> = entries
                .iter()
                .map(|e| e.to_value(&input.base_host))
                .collect();
            let mut obj = Map::new();
            obj.insert("links".into(), Value::Array(links));
            Value::Object(obj)
        }
        FuzzBody::ResourceLinksArray(entries) => {
            let key = response_key.clone().unwrap_or(Cow::Borrowed("resource"));
            let links: Vec<Value> = entries
                .iter()
                .map(|e| e.to_value(&input.base_host))
                .collect();
            let mut obj = Map::new();
            obj.insert(format!("{key}_links"), Value::Array(links));
            Value::Object(obj)
        }
        FuzzBody::LinksDict { next } => {
            let mut links = Map::new();
            links.insert(
                "next".into(),
                next.clone().map(Value::String).unwrap_or(Value::Null),
            );
            let mut obj = Map::new();
            obj.insert("links".into(), Value::Object(links));
            Value::Object(obj)
        }
        FuzzBody::NextField(next) => {
            let mut obj = Map::new();
            obj.insert(
                "next".into(),
                next.as_ref()
                    .map(|n| Value::String(n.render(&input.base_host)))
                    .unwrap_or(Value::Null),
            );
            Value::Object(obj)
        }
    };

    let _ = openstack_sdk::api::fuzz_next_page_from_body(&content, &response_key, base_endpoint);
});
