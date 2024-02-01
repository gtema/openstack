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

//! Params
//!

use std::borrow::Cow;

use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

use serde_json::{json, Value};

use crate::api::BodyError;

/// A trait representing a parameter value.
pub trait ParamValue<'a> {
    #[allow(clippy::wrong_self_convention)]
    /// The parameter value as a string.
    fn as_value(&self) -> Cow<'a, str>;
}

impl ParamValue<'static> for bool {
    fn as_value(&self) -> Cow<'static, str> {
        if *self {
            "true".into()
        } else {
            "false".into()
        }
    }
}

impl<'a> ParamValue<'a> for &'a str {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl ParamValue<'static> for String {
    fn as_value(&self) -> Cow<'static, str> {
        self.clone().into()
    }
}

impl<'a> ParamValue<'a> for &'a String {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).into()
    }
}

impl<'a> ParamValue<'a> for Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        self.clone()
    }
}

impl<'a, 'b: 'a> ParamValue<'a> for &'b Cow<'a, str> {
    fn as_value(&self) -> Cow<'a, str> {
        (*self).clone()
    }
}

impl ParamValue<'static> for i32 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for u32 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for u64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for f32 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for f64 {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl ParamValue<'static> for DateTime<Utc> {
    fn as_value(&self) -> Cow<'static, str> {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            .into()
    }
}

impl ParamValue<'static> for NaiveDate {
    fn as_value(&self) -> Cow<'static, str> {
        format!("{}", self.format("%Y-%m-%d")).into()
    }
}

/// A structure for query parameters.
#[derive(Debug, Default, Clone)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    /// Push a single parameter.
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    /// Push a single parameter.
    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    /// Push a set of parameters.
    pub fn extend<'b, I, K, V>(&mut self, iter: I) -> &mut Self
    where
        I: Iterator<Item = (K, V)>,
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params
            .extend(iter.map(|(key, value)| (key.into(), value.as_value())));
        self
    }

    /// Add the parameters to a URL.
    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}

/// A structure for form parameters.
#[derive(Debug, Default, Clone)]
pub struct JsonBodyParams {
    data: Value,
}

impl JsonBodyParams {
    /// Push a single parameter.
    pub fn push<K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<String>,
        V: serde::Serialize,
    {
        match self.data.as_object_mut() {
            Some(ref mut m) => {
                m.insert(key.into(), json!(value));
            }
            None => {
                self.data = json!({key.into(): value});
            }
        }
        self
    }

    /// Push a single optional parameter.
    pub fn push_opt<K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<String>,
        V: serde::Serialize,
    {
        if let Some(value) = value {
            match self.data.as_object_mut() {
                Some(ref mut m) => {
                    m.insert(key.into(), json!(value));
                }
                None => {
                    self.data = json!({key.into(): value});
                }
            }
        }
        self
    }

    /// Encode the parameters into a request body.
    pub fn into_body(self) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let body = self.data.to_string();
        Ok(Some(("application/json", body.into_bytes())))
    }
    ///
    /// Encode the parameters into a request body.
    pub fn into_body_with_root_key(
        self,
        key: &'static str,
    ) -> Result<Option<(&'static str, Vec<u8>)>, BodyError> {
        let body = json!({ key: self.data}).to_string();
        Ok(Some(("application/json", body.into_bytes())))
    }
}

#[cfg(test)]
mod tests {
    use super::JsonBodyParams;
    use crate::api::ParamValue;
    use serde_json::json;

    #[test]
    fn bool_str() {
        let items = &[(true, "true"), (false, "false")];

        for (i, s) in items {
            assert_eq!((*i).as_value(), *s);
        }
    }

    #[test]
    fn json_body() {
        let mut data = JsonBodyParams::default();
        data.push("foo", "bar")
            .push("foo1", "bar1")
            .push("foo_bool", true)
            .push("foo_array", vec!["a", "b"]);
        let (mime, body) = data.into_body().unwrap().unwrap();
        assert_eq!("application/json", mime);
        assert_eq!(
            std::str::from_utf8(&body).unwrap(),
            json!({"foo": "bar", "foo1": "bar1", "foo_bool": true, "foo_array": ["a", "b"]})
                .to_string()
        );
    }
}
