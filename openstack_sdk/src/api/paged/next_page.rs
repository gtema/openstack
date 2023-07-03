//! Module to detect next page URL from the response
use anyhow::Context;
use serde_json::Value;
use std::borrow::Cow;
use thiserror::Error;
use tracing::debug;
use url::Url;

use http::HeaderMap;

use crate::api::PaginationError;

///
/// LinkHeader struct
#[derive(Debug)]
struct LinkHeader<'a> {
    url: &'a str,
    params: Vec<(&'a str, &'a str)>,
}
pub(crate) fn next_page_from_headers(headers: &HeaderMap) -> Result<Option<Url>, PaginationError> {
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
    let mut next_url: Option<&str>;
    if content.is_object() {
        let mut next: Option<&Value> = None;
        let mut pagination_key = "links";
        // First try to get "links" element
        let mut links = content.get(pagination_key);
        if links.is_none() {
            if let Some(rk) = response_key {
                // Nova has instead `<resource_key>_links`
                links = content.get(format!("{}_links", rk));
            }
        }
        if let Some(mut v) = links {
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
                    return Err(PaginationError::Body {
                        msg: "`rel=next` is missing in links".into(),
                    });
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
    use serde_json::{json, Value};
    use std::borrow::Cow;
    use url::Url;

    use crate::api::paged::next_page::next_page_from_body;
    use crate::api::PaginationError;

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
        let err =
            next_page_from_body(&data, &None, Url::parse("http://dummy").unwrap()).unwrap_err();
        if let PaginationError::Body { msg } = err {
            // expected error
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn test_body_links_relative() {
        let data = json!({"next": "/foo/bar"});
        let res =
            next_page_from_body(&data, &None, Url::parse("http://dummy:15").unwrap()).unwrap();
        assert_eq!(res.unwrap(), Url::parse("http://dummy:15/foo/bar").unwrap());
    }
}
