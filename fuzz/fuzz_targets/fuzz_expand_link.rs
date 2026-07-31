#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use url::Url;

extern crate openstack_sdk;

// `expand_link` resolves a `href` from a version-discovery document into an
// absolute URL relative to a base URL, with recovery paths for a malformed
// port (regex-based) and for relative URLs without a scheme/host. Random
// bytes rarely produce a string that even looks like `scheme://host:port/path`,
// so we model the shapes the function actually branches on and let the
// fuzzer mutate within them.

#[derive(Debug, Arbitrary)]
enum FuzzScheme {
    Http,
    Https,
}

impl FuzzScheme {
    fn as_str(&self) -> &'static str {
        match self {
            FuzzScheme::Http => "http",
            FuzzScheme::Https => "https",
        }
    }
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
enum FuzzLink {
    /// A path with no scheme/host, e.g. "/v2/foo" -> RelativeUrlWithoutBase.
    Relative(String),
    /// A well-formed absolute URL pointing at the same host as the base.
    AbsoluteSameHost(String),
    /// A well-formed absolute URL pointing at a different host.
    AbsoluteOtherHost(FuzzScheme, FuzzHost, String),
    /// `scheme://host:<non-numeric-port>/path` -> triggers the InvalidPort
    /// regex-recovery branch.
    BadPort(FuzzScheme, FuzzHost, u16, String),
    /// Fully arbitrary text, to explore other/unexpected parse errors.
    Raw(String),
}

impl FuzzLink {
    fn render(&self, own_scheme: &FuzzScheme, own_host: &FuzzHost) -> String {
        match self {
            FuzzLink::Relative(path) => format!("/{path}"),
            FuzzLink::AbsoluteSameHost(path) => {
                format!("{}://{}/{path}", own_scheme.as_str(), own_host.as_str())
            }
            FuzzLink::AbsoluteOtherHost(scheme, host, path) => {
                format!("{}://{}/{path}", scheme.as_str(), host.as_str())
            }
            FuzzLink::BadPort(scheme, host, port, path) => {
                // Append a non-digit so the port segment fails to parse as u16.
                format!("{}://{}:{port}x/{path}", scheme.as_str(), host.as_str())
            }
            FuzzLink::Raw(s) => s.clone(),
        }
    }
}

#[derive(Debug, Arbitrary)]
struct FuzzInput {
    link: FuzzLink,
    base_scheme: FuzzScheme,
    base_host: FuzzHost,
    base_port: Option<u16>,
    base_path: String,
    service_type: String,
}

fuzz_target!(|input: FuzzInput| {
    let base_url_str = match input.base_port {
        Some(port) => format!(
            "{}://{}:{port}/{}",
            input.base_scheme.as_str(),
            input.base_host.as_str(),
            input.base_path
        ),
        None => format!(
            "{}://{}/{}",
            input.base_scheme.as_str(),
            input.base_host.as_str(),
            input.base_path
        ),
    };
    let Ok(base_url) = Url::parse(&base_url_str) else {
        return;
    };

    let link = input.link.render(&input.base_scheme, &input.base_host);
    let _ = openstack_sdk::catalog::expand_link(&link, &base_url, &input.service_type);
});
