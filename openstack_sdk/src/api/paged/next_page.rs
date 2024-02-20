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

//! Module to detect next page URL from the response

use serde_json::Value;
use std::borrow::Cow;

use url::Url;

use http::HeaderMap;

use crate::api::PaginationError;

pub(crate) fn next_page_from_headers(_headers: &HeaderMap) -> Result<Option<Url>, PaginationError> {
    Err(PaginationError::Body {
        msg: "error".into(),
    })
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

    #[test]
    fn test_body_links() {
        let data = json!({"links": [{"rel": "next", "href": "http://foo.bar"}]});
        let res = next_page_from_body(&data, &None, Url::parse("http://dummy").unwrap());
        assert_eq!(res.unwrap().unwrap(), Url::parse("http://foo.bar").unwrap());
    }

    #[test]
    fn test_body_nova_links() {
        let data = json!({"flavors_links": [{"rel": "next", "href": "http://foo.bar"}]});
        let key: Cow<'static, str> = Cow::Owned("flavors".to_string());
        let res = next_page_from_body(&data, &Some(key), Url::parse("http://dummy").unwrap());
        assert_eq!(res.unwrap().unwrap(), Url::parse("http://foo.bar").unwrap());
    }

    #[test]
    fn test_body_no_links() {
        let data = json!({});
        let key: Cow<'static, str> = Cow::Owned("flavors".to_string());
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
}
