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

//! SSRF address-denylist checking for `idp_http_request`'s guest-chosen
//! origin. Unlike `identity_http_request`'s origin (host-resolved from the
//! user's own cloud config, never guest-influenced), `idp_http_request`'s
//! origin comes from a value the guest itself returned
//! (`sso_build_request`'s `url` field) — so before the host ever connects to
//! it, the resolved address must be checked against this denylist, and
//! re-checked immediately before every actual request (not just once), to
//! close the DNS-rebinding gap where a hostname resolves to a safe address
//! at validation time and an unsafe one later.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, ToSocketAddrs};

/// True if `ip` falls in any range a host-mediated plugin request must never
/// reach: loopback, link-local (including the cloud metadata address),
/// private, multicast/reserved, or unspecified — IPv4 and IPv6, with
/// IPv4-mapped IPv6 addresses unwrapped and checked against the IPv4 ranges.
pub(crate) fn is_denylisted(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_denylisted_v4(v4),
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(mapped) => is_denylisted_v4(mapped),
            None => is_denylisted_v6(v6),
        },
    }
}

fn is_denylisted_v4(ip: Ipv4Addr) -> bool {
    ip.octets()[0] == 0 // 0.0.0.0/8 (unspecified block)
        || ip.is_loopback() // 127.0.0.0/8
        || ip.is_link_local() // 169.254.0.0/16 (covers 169.254.169.254)
        || ip.is_private() // 10.0.0.0/8, 172.16.0.0/12, 192.168.0.0/16
        || ip.is_multicast() // 224.0.0.0/4
        || ip.octets()[0] >= 240 // 240.0.0.0/4 (reserved)
}

fn is_denylisted_v6(ip: Ipv6Addr) -> bool {
    ip.is_loopback() // ::1
        || ip.is_unique_local() // fc00::/7
        || ip.is_unicast_link_local() // fe80::/10
        || ip.is_multicast() // ff00::/8
        || ip.is_unspecified() // ::
}

/// The enforcement point `resolve_and_check` actually calls. Under the
/// `fuzzing` feature only — never compiled into a release `osc` binary,
/// since `fuzzing` isn't a default feature — loopback is exempted so
/// integration tests can point `idp_http_request` at a local mock server
/// (e.g. `httpmock`, which always binds `127.0.0.1`). Every other
/// denylisted range remains enforced even under this feature; `is_denylisted`
/// itself is untouched and still reports loopback as denylisted for its own
/// (Task 1) unit tests and any future fuzz target.
#[cfg(feature = "fuzzing")]
fn is_denylisted_for_enforcement(ip: IpAddr) -> bool {
    if ip.is_loopback() {
        return false;
    }
    is_denylisted(ip)
}

#[cfg(not(feature = "fuzzing"))]
fn is_denylisted_for_enforcement(ip: IpAddr) -> bool {
    is_denylisted(ip)
}

/// Resolve `host:port` and reject if it resolves to *any* denylisted
/// address — never pick-and-choose among multiple resolved addresses.
/// Blocking (real DNS I/O); callers must run this off the async runtime.
pub(crate) fn resolve_and_check(host: &str, port: u16) -> Result<(), String> {
    let addrs: Vec<_> = (host, port)
        .to_socket_addrs()
        .map_err(|e| format!("failed to resolve `{host}`: {e}"))?
        .collect();
    if addrs.is_empty() {
        return Err(format!("`{host}` did not resolve to any address"));
    }
    for addr in &addrs {
        if is_denylisted_for_enforcement(addr.ip()) {
            return Err(format!(
                "`{host}` resolves to a disallowed address ({}); refusing to connect",
                addr.ip()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v4_loopback_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))));
    }

    #[test]
    fn v4_link_local_metadata_address_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(169, 254, 169, 254))));
    }

    #[test]
    fn v4_private_10_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))));
    }

    #[test]
    fn v4_private_172_16_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 1))));
    }

    #[test]
    fn v4_private_192_168_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1))));
    }

    #[test]
    fn v4_multicast_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(224, 0, 0, 1))));
    }

    #[test]
    fn v4_reserved_240_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(240, 0, 0, 1))));
    }

    #[test]
    fn v4_unspecified_block_is_denylisted() {
        assert!(is_denylisted(IpAddr::V4(Ipv4Addr::new(0, 1, 2, 3))));
    }

    #[test]
    fn v6_loopback_is_denylisted() {
        assert!(is_denylisted(IpAddr::V6(Ipv6Addr::LOCALHOST)));
    }

    #[test]
    fn v6_unique_local_is_denylisted() {
        assert!(is_denylisted(IpAddr::V6(Ipv6Addr::new(
            0xfc00, 0, 0, 0, 0, 0, 0, 1
        ))));
    }

    #[test]
    fn v6_link_local_is_denylisted() {
        assert!(is_denylisted(IpAddr::V6(Ipv6Addr::new(
            0xfe80, 0, 0, 0, 0, 0, 0, 1
        ))));
    }

    #[test]
    fn v6_multicast_is_denylisted() {
        assert!(is_denylisted(IpAddr::V6(Ipv6Addr::new(
            0xff00, 0, 0, 0, 0, 0, 0, 1
        ))));
    }

    #[test]
    fn v6_unspecified_is_denylisted() {
        assert!(is_denylisted(IpAddr::V6(Ipv6Addr::UNSPECIFIED)));
    }

    /// `::ffff:169.254.169.254` — an IPv4-mapped IPv6 address wrapping the
    /// cloud metadata address; must be caught via the unwrapped check.
    #[test]
    fn v6_ipv4_mapped_metadata_address_is_denylisted() {
        let mapped = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xa9fe, 0xa9fe);
        assert!(is_denylisted(IpAddr::V6(mapped)));
    }

    #[test]
    fn public_v4_address_passes() {
        assert!(!is_denylisted(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))));
    }

    #[test]
    fn public_v6_address_passes() {
        assert!(!is_denylisted(IpAddr::V6(Ipv6Addr::new(
            0x2606, 0x4700, 0x4700, 0, 0, 0, 0, 0x1111
        ))));
    }
}
