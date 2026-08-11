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

//! Selecting the right `BODY_SCHEMA` among a microversioned operation's
//! vendored variants.
//!
//! Generated code emits one request struct per microversion break (e.g.
//! `create_20`, `create_233`, ...), each carrying its own `BODY_SCHEMA`
//! (see [`crate::api::rest_endpoint::RestEndpoint::min_version`] /
//! `max_version` for the equivalent struct-selection bounds). Consumers that
//! need a schema *before* committing to a specific variant struct -- e.g. to
//! render an editor template or validate a YAML buffer against the right
//! shape -- use [`select_schema`] against the cloud's discovered version
//! instead.

use crate::api::rest_endpoint::version_range_compatible;
use crate::types::ApiVersion;

/// One microversion-scoped request-body schema, as vendored from a
/// generated `create_NN.rs`-style module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchemaVariant {
    /// The microversion this variant was introduced at. `None` means it
    /// applies from the service's unversioned/earliest floor.
    pub min_version: Option<ApiVersion>,
    /// The microversion this variant stops applying at (inclusive). `None`
    /// means it is still current -- valid for every version at or above
    /// `min_version`.
    pub max_version: Option<ApiVersion>,
    /// The `BODY_SCHEMA` constant for this variant.
    pub schema: &'static str,
}

/// Select the schema whose microversion range covers `negotiated`.
///
/// When multiple variants match (their ranges overlap `negotiated`), the one
/// with the highest `min_version` wins -- the most specific/newest
/// applicable variant. Returns `None` when no variant covers `negotiated`,
/// including the empty-slice case.
///
/// A single-variant slice (the common case today -- most operations have
/// exactly one `BODY_SCHEMA`) degrades to always returning that one schema,
/// since an unbounded `[None, None]` range is compatible with anything.
pub fn select_schema(variants: &[SchemaVariant], negotiated: ApiVersion) -> Option<&'static str> {
    variants
        .iter()
        .filter(|v| {
            version_range_compatible(
                v.min_version.unwrap_or(ApiVersion::new(0, 0)),
                v.max_version,
                Some(negotiated),
                Some(negotiated),
            )
        })
        .max_by_key(|v| v.min_version.unwrap_or(ApiVersion::new(0, 0)))
        .map(|v| v.schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(major: u8, minor: u8) -> ApiVersion {
        ApiVersion::new(major, minor)
    }

    #[test]
    fn empty_slice_selects_nothing() {
        assert_eq!(select_schema(&[], v(2, 5)), None);
    }

    #[test]
    fn single_unbounded_variant_always_matches() {
        let variants = [SchemaVariant {
            min_version: None,
            max_version: None,
            schema: "SG_RULE",
        }];
        assert_eq!(select_schema(&variants, v(2, 0)), Some("SG_RULE"));
        assert_eq!(select_schema(&variants, v(2, 99)), Some("SG_RULE"));
    }

    #[test]
    fn picks_highest_matching_non_overlapping_variant() {
        let variants = [
            SchemaVariant {
                min_version: Some(v(2, 0)),
                max_version: Some(v(2, 32)),
                schema: "CREATE_20",
            },
            SchemaVariant {
                min_version: Some(v(2, 33)),
                max_version: Some(v(2, 66)),
                schema: "CREATE_233",
            },
            SchemaVariant {
                min_version: Some(v(2, 67)),
                max_version: None,
                schema: "CREATE_267",
            },
        ];
        assert_eq!(select_schema(&variants, v(2, 1)), Some("CREATE_20"));
        assert_eq!(select_schema(&variants, v(2, 40)), Some("CREATE_233"));
        assert_eq!(select_schema(&variants, v(2, 90)), Some("CREATE_267"));
    }

    #[test]
    fn version_below_every_variant_selects_nothing() {
        let variants = [SchemaVariant {
            min_version: Some(v(2, 33)),
            max_version: None,
            schema: "CREATE_233",
        }];
        assert_eq!(select_schema(&variants, v(2, 0)), None);
    }

    #[test]
    fn unbounded_latest_variant_wins_ties_via_highest_min_version() {
        let variants = [
            SchemaVariant {
                min_version: Some(v(2, 0)),
                max_version: None,
                schema: "OLD",
            },
            SchemaVariant {
                min_version: Some(v(2, 50)),
                max_version: None,
                schema: "NEW",
            },
        ];
        // Both ranges are open-ended and both cover 2.80; the newer variant
        // (higher min_version) is the one actually in effect.
        assert_eq!(select_schema(&variants, v(2, 80)), Some("NEW"));
    }
}
