#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use openstack_sdk_core::catalog::ServiceEndpoint;
use openstack_sdk_core::types::ApiVersion;
use url::Url;

extern crate openstack_sdk_core;

// `ServiceEndpoint::build_request_url` combines a server-supplied catalog URL with a
// user-supplied endpoint path, stripping/re-appending a trailing project-ID path
// segment to avoid doubling it. This target previously found a real panic: when
// `endpoint` starts with `pid_suffix` but the byte right after the prefix begins a
// multi-byte UTF-8 character, the old byte-offset slice landed mid-codepoint. Now
// fixed via char-aware `strip_prefix`; this target guards against regressions and
// any other edge case in the segment bookkeeping.

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
enum FuzzEndpoint {
    /// Arbitrary text unrelated to the pid_suffix.
    Raw(String),
    /// Exactly the pid_suffix.
    ExactSuffix,
    /// pid_suffix + '/' + arbitrary text (the "normal" doubled-segment case).
    SuffixSlashThen(String),
    /// pid_suffix immediately followed by arbitrary text with no separator
    /// (the shape that used to trigger the multi-byte-boundary panic).
    SuffixThen(String),
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    base_host: FuzzHost,
    base_path: String,
    pid_suffix: Option<String>,
    endpoint: FuzzEndpoint,
    major: u8,
    minor: u8,
}

fuzz_target!(|input: FuzzInput| {
    let base_url_str = format!("http://{}/{}", input.base_host.as_str(), input.base_path);
    let Ok(base_url) = Url::parse(&base_url_str) else {
        return;
    };
    if base_url.cannot_be_a_base() {
        return;
    }

    let mut endpoint = ServiceEndpoint::new(base_url, ApiVersion::new(input.major, input.minor));
    endpoint.set_last_segment_with_project_id(input.pid_suffix.clone());

    let endpoint_path = match &input.endpoint {
        FuzzEndpoint::Raw(s) => s.clone(),
        FuzzEndpoint::ExactSuffix => input.pid_suffix.clone().unwrap_or_default(),
        FuzzEndpoint::SuffixSlashThen(rest) => {
            format!("{}/{rest}", input.pid_suffix.clone().unwrap_or_default())
        }
        FuzzEndpoint::SuffixThen(rest) => {
            format!("{}{rest}", input.pid_suffix.clone().unwrap_or_default())
        }
    };

    let _ = endpoint.build_request_url(&endpoint_path);
});
