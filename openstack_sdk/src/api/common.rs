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

use secrecy::{ExposeSecret, SecretString};
use std::borrow::Cow;
use std::fmt;
use std::iter;
use std::ops;

use itertools::Itertools;

use crate::api::ParamValue;

/// A comma-separated list of values.
#[derive(Debug, Clone, Default)]
pub struct CommaSeparatedList<T> {
    data: Vec<T>,
}

impl<T> CommaSeparatedList<T> {
    /// Create a new, empty comma-separated list.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl<T> From<Vec<T>> for CommaSeparatedList<T> {
    fn from(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T> iter::FromIterator<T> for CommaSeparatedList<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        Self {
            data: iter.into_iter().collect(),
        }
    }
}

impl<T> ops::Deref for CommaSeparatedList<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> ops::DerefMut for CommaSeparatedList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> fmt::Display for CommaSeparatedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data.iter().format(","))
    }
}

impl<'a, T> ParamValue<'a> for CommaSeparatedList<T>
where
    T: ParamValue<'a>,
{
    fn as_value(&self) -> Cow<'a, str> {
        format!("{}", self.data.iter().map(|d| d.as_value()).format(",")).into()
    }
}

impl<'a, T> ParamValue<'a> for &CommaSeparatedList<T>
where
    T: ParamValue<'a>,
{
    fn as_value(&self) -> Cow<'a, str> {
        format!("{}", self.data.iter().map(|d| d.as_value()).format(",")).into()
    }
}

/// Serialize `SecretString` as string
pub fn serialize_sensitive_optional_string<S>(
    value: &Option<SecretString>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(val) => serializer.serialize_str(val.expose_secret()),
        None => serializer.serialize_none(),
    }
}

/// Serialize `Option<SecretString>` as string
pub fn serialize_sensitive_string<S>(value: &SecretString, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(value.expose_secret())
}
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use std::iter;

    use crate::api::common::CommaSeparatedList;
    use crate::api::params::ParamValue;

    #[test]
    fn comma_separated_list_default() {
        let csl = CommaSeparatedList::<u64>::default();
        assert!(csl.is_empty());
    }

    #[test]
    fn comma_separated_list_vec() {
        let csl = CommaSeparatedList::<u64>::new();
        let _: &Vec<u64> = &csl;
    }

    #[test]
    fn comma_separated_list_from_iter() {
        let _: CommaSeparatedList<_> = iter::once(2).collect();
    }

    #[test]
    fn comma_separated_list_display() {
        let csl_one: CommaSeparatedList<_> = iter::once(2).collect();
        assert_eq!(csl_one.to_string(), "2");
        let csl_two: CommaSeparatedList<_> = [1, 2].iter().copied().collect();
        assert_eq!(csl_two.to_string(), "1,2");
    }

    #[test]
    fn comma_separated_list_param_value() {
        let csl_one: CommaSeparatedList<_> = iter::once(2).collect();
        assert_eq!(csl_one.as_value(), "2");
        let csl_two: CommaSeparatedList<_> = [1, 2].iter().copied().collect();
        assert_eq!(csl_two.as_value(), "1,2");
        let csl_str_one: CommaSeparatedList<Cow<str>> = iter::once("one".into()).collect();
        assert_eq!(csl_str_one.as_value(), "one");
        let csl_str_two: CommaSeparatedList<Cow<str>> =
            ["one".into(), "two".into()].iter().cloned().collect();
        assert_eq!(csl_str_two.as_value(), "one,two");
    }
}
