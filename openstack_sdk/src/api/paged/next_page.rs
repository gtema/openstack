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

//! Module to detect next page URL from the server response

use http::HeaderMap;
use serde_json::Value;
use std::borrow::Cow;
use thiserror::Error;
use url::Url;

use crate::api::PaginationError;

#[derive(Debug)]
struct LinkHeader<'a> {
    url: &'a str,
    params: Vec<(&'a str, &'a str)>,
}

impl<'a> LinkHeader<'a> {
    fn parse(s: &'a str) -> Result<Self, LinkHeaderParseError> {
        let mut parts = s.split(';');

        let url_part = parts.next().expect("a split always has at least one part");
        let url = {
            let part = url_part.trim();
            if part.starts_with('<') && part.ends_with('>') {
                &part[1..part.len() - 1]
            } else {
                return Err(LinkHeaderParseError::NoBrackets);
            }
        };

        let params = parts
            .map(|part| {
                let part = part.trim();
                let mut halves = part.splitn(2, '=');
                let key = halves.next().expect("a split always has at least one part");
                let value = if let Some(value) = halves.next() {
                    if value.starts_with('"') && value.ends_with('"') {
                        &value[1..value.len() - 1]
                    } else {
                        value
                    }
                } else {
                    return Err(LinkHeaderParseError::MissingParamValue);
                };

                Ok((key, value))
            })
            .collect::<Result<Vec<_>, LinkHeaderParseError>>()?;

        Ok(Self { url, params })
    }
}

/// An error which can occur when parsing a link header.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LinkHeaderParseError {
    /// An invalid HTTP header found.
    #[error("invalid header")]
    InvalidHeader {
        /// The source of the error.
        #[from]
        source: reqwest::header::ToStrError,
    },
    /// The `url` for a `Link` header missing `<>` brackets.
    #[error("missing brackets around url")]
    NoBrackets,
    /// A parameter for a `Link` header missing a value.
    #[error("missing parameter value")]
    MissingParamValue,
}

impl LinkHeaderParseError {
    fn invalid_header(source: reqwest::header::ToStrError) -> Self {
        Self::InvalidHeader { source }
    }
}

pub(crate) fn next_page_from_headers(headers: &HeaderMap) -> Result<Option<Url>, PaginationError> {
    let link_headers = headers.get_all(reqwest::header::LINK).iter();
    link_headers
        .map(|link| {
            let value = link
                .to_str()
                .map_err(LinkHeaderParseError::invalid_header)?;
            Ok(LinkHeader::parse(value)?)
        })
        .collect::<Result<Vec<_>, PaginationError>>()?
        .into_iter()
        .find_map(|header| {
            let is_next_link = header
                .params
                .into_iter()
                .any(|(key, value)| key == "rel" && value == "next");

            if is_next_link {
                Some(header.url.parse().map_err(|x| PaginationError::InvalidUrl {
                    url: header.url.to_string(),
                    source: x,
                }))
            } else {
                None
            }
        })
        .transpose()
}

/// Detect link to the next page from the response body.
pub(crate) fn next_page_from_body(
    content: &Value,
    response_key: &Option<Cow<'_, str>>,
    base_endpoint: Url,
) -> Result<Option<Url>, PaginationError> {
    let mut _next_url: Option<&str>;
    if content.is_object() {
        let mut next: Option<&Value> = None;
        let pagination_key = "links";
        // First try to get "links" element
        let mut links = content.get(pagination_key);
        if links.is_none() {
            if let Some(rk) = response_key {
                // Nova has instead `<resource_key>_links`
                links = content.get(format!("{}_links", rk));
            }
        }
        if let Some(v) = links {
            // Sometimes "links" is just a dict
            // {
            //   next: next_link,
            //   prev: prev_link,
            //   curr: curr_link
            // }
            if v.is_array() {
                for link_el in v.as_array().unwrap() {
                    if link_el.is_object() {
                        match link_el.get("rel") {
                            Some(rel) => {
                                if rel == "next" {
                                    next = link_el.get("href");
                                    break;
                                }
                            }
                            None => {
                                return Err(PaginationError::Body {
                                    msg: "`rel` element is missing in links".into(),
                                })
                            }
                        }
                    }
                }
                if next.is_none() {
                    // Pagination is present, but there is no info about next
                    // page. Means there is no next page
                    return Ok(None);
                }
            }
        } else {
            // Links is missing
            // Glance has a next field in the main body
            next = content.get("next");
        }

        if let Some(n) = next {
            // We expect that the link contains all initial query parameters and we do NOT read them.
            if let Some(next_url) = n.as_str() {
                let next: String = if !next_url.starts_with("http") {
                    // Some services (i.e. Glance) has a relative link without
                    // domain. So we need to construct it back.
                    String::from(base_endpoint.scheme())
                        + "://"
                        + base_endpoint.domain().expect("Domain is present")
                        + ":"
                        + &base_endpoint
                            .port_or_known_default()
                            .expect("Port is unknown")
                            .to_string()
                        + next_url
                } else {
                    next_url.to_string()
                };
                return Some(Url::parse(&next).map_err(|x| PaginationError::InvalidUrl {
                    source: x,
                    url: next,
                }))
                .transpose();
            }
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::borrow::Cow;
    use url::Url;

    use crate::api::paged::next_page::next_page_from_body;

    use super::*;

    #[test]
    fn test_body_links() {
        let data = json!({"links": [{"rel": "next", "href": "http://foo.bar"}]});
        let res = next_page_from_body(&data, &None, Url::parse("http://dummy").unwrap());
        assert_eq!(res.unwrap().unwrap(), Url::parse("http://foo.bar").unwrap());
    }

    #[test]
    fn test_body_nova_links() {
        let data = json!({"flavors_links": [{"rel": "next", "href": "http://foo.bar"}]});
        let key: Cow<'static, str> = Cow::Owned("flavors".into());
        let res = next_page_from_body(&data, &Some(key), Url::parse("http://dummy").unwrap());
        assert_eq!(res.unwrap().unwrap(), Url::parse("http://foo.bar").unwrap());
    }

    #[test]
    fn test_body_no_links() {
        let data = json!({});
        let key: Cow<'static, str> = Cow::Owned("flavors".into());
        let res = next_page_from_body(&data, &Some(key), Url::parse("http://dummy").unwrap());
        assert_eq!(res.unwrap(), None);
    }

    #[test]
    fn test_body_links_no_next_rel() {
        let data = json!({"links": [{"rel": "curr", "href": "http://foo1.bar"}]});
        assert_eq!(
            None,
            next_page_from_body(&data, &None, Url::parse("http://dummy").unwrap()).unwrap()
        );
    }

    #[test]
    fn test_body_links_relative() {
        let data = json!({"next": "/foo/bar"});
        let res =
            next_page_from_body(&data, &None, Url::parse("http://dummy:15").unwrap()).unwrap();
        assert_eq!(res.unwrap(), Url::parse("http://dummy:15/foo/bar").unwrap());
    }

    #[test]
    fn test_link_header_no_brackets() {
        let err = LinkHeader::parse("url; param=value").unwrap_err();
        if let LinkHeaderParseError::NoBrackets = err {
            // expected error
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_link_header_no_param_value() {
        let err = LinkHeader::parse("<url>; param").unwrap_err();
        if let LinkHeaderParseError::MissingParamValue = err {
            // expected error
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_link_header_no_params() {
        let link = LinkHeader::parse("<url>").unwrap();
        assert_eq!(link.url, "url");
        assert_eq!(link.params.len(), 0);
    }

    #[test]
    fn test_link_header_quoted_params() {
        let link = LinkHeader::parse("<url>; param=\"value\"; param2=\"value\"").unwrap();
        assert_eq!(link.url, "url");
        assert_eq!(link.params.len(), 2);
        assert_eq!(link.params[0].0, "param");
        assert_eq!(link.params[0].1, "value");
        assert_eq!(link.params[1].0, "param2");
        assert_eq!(link.params[1].1, "value");
    }
    #[test]
    fn test_link_header_bare_params() {
        let link = LinkHeader::parse("<url>; param=value; param2=value").unwrap();
        assert_eq!(link.url, "url");
        assert_eq!(link.params.len(), 2);
        assert_eq!(link.params[0].0, "param");
        assert_eq!(link.params[0].1, "value");
        assert_eq!(link.params[1].0, "param2");
        assert_eq!(link.params[1].1, "value");
    }
}
