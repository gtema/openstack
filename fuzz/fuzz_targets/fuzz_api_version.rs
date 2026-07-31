#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use openstack_sdk::types::ApiVersion;
use url::Url;

extern crate openstack_sdk;

// `ApiVersion::from_apiver_str`/`from_url` are already fully public, so no
// SDK-side changes were needed for this target. `from_apiver_str` is a
// regex-based single-token parser (`v2.3`, `2.3`, `v1`); `from_url` walks a
// URL's path segments, heuristically strips a trailing project-ID-looking
// segment, and delegates to `from_apiver_str`.

#[derive(Debug, Arbitrary)]
enum FuzzHost {
    FooBar,
    ExampleOrg,
}

impl FuzzHost {
    fn as_str(&self) -> &'static str {
        match self {
            FuzzHost::FooBar => "foo.bar",
            FuzzHost::ExampleOrg => "example.org",
        }
    }
}

#[derive(Debug, Arbitrary)]
enum FuzzInput {
    /// Directly fuzz the single-token parser with both prefix modes.
    ApiverStr { data: String, prefixed: bool },
    /// Fuzz the URL/path-segment-stripping logic.
    Url {
        host: FuzzHost,
        path_segments: Vec<String>,
        project_id: Option<String>,
    },
}

fuzz_target!(|input: FuzzInput| {
    match input {
        FuzzInput::ApiverStr { data, prefixed } => {
            let _ = ApiVersion::from_apiver_str(&data, prefixed);
        }
        FuzzInput::Url {
            host,
            path_segments,
            project_id,
        } => {
            let path = path_segments.join("/");
            let Ok(url) = Url::parse(&format!("http://{}/{path}", host.as_str())) else {
                return;
            };
            let _ = ApiVersion::from_url(&url, project_id.as_deref());
        }
    }
});
