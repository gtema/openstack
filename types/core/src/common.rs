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

//! Common types that can be used in responses of the API operations
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};
use std::fmt;
use std::str::FromStr;

/// Deserialize whatever is an integer, number or a string number falling back to the Default (0)
pub fn deser_num_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr + Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumOrString<T> {
        String(String),
        Number(T),
    }

    Ok(match NumOrString::<T>::deserialize(deserializer) {
        Ok(NumOrString::String(s)) => s.parse::<T>().unwrap_or_else(|_| T::default()),
        Ok(NumOrString::Number(num)) => <T as Into<T>>::into(num),
        Err(_) => T::default(),
    })
}

/// Deserialize whatever is an integer, number or a string number falling back to the Default (0)
/// as an optional attribute
pub fn deser_num_str_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: FromStr + Deserialize<'de> + Default,
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumOrString<T> {
        String(String),
        Number(T),
        Null,
    }

    Ok(match NumOrString::<T>::deserialize(deserializer) {
        Ok(NumOrString::String(s)) => Some(s.parse::<T>().unwrap_or_else(|_| T::default())),
        Ok(NumOrString::Number(num)) => Some(<T as Into<T>>::into(num)),
        _ => None,
    })
}

/// Deserialize whatever is a boolean or a string boolean falling back to the Default (false)
pub fn deser_bool_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrStr {
        String(String),
        Bool(bool),
    }

    Ok(match BoolOrStr::deserialize(deserializer) {
        Ok(BoolOrStr::String(s)) => s.parse().unwrap_or(false),
        Ok(BoolOrStr::Bool(val)) => val,
        Err(_) => false,
    })
}

/// Deserialize whatever is a boolean or a string boolean falling back to the Default (false) as an
/// optional attribute
pub fn deser_bool_str_opt<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BoolOrStr {
        String(String),
        Bool(bool),
        Null,
    }

    Ok(match BoolOrStr::deserialize(deserializer) {
        Ok(BoolOrStr::String(s)) => Some(s.parse().unwrap_or(false)),
        Ok(BoolOrStr::Bool(val)) => Some(val),
        _ => None,
    })
}

/// IntString
///
/// Integer type holder that can be deserialized from Integer or String
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct IntString(u64);

impl fmt::Display for IntString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for IntString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl Visitor<'_> for MyVisitor {
            type Value = IntString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(IntString(val))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Ok(IntString(0)),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

/// NumString
///
/// Number type holder that can be deserialized from Number or String
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct NumString(f64);

impl fmt::Display for NumString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl<'de> Deserialize<'de> for NumString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl Visitor<'_> for MyVisitor {
            type Value = NumString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("number or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumString(val as f64))
            }

            fn visit_f64<E>(self, val: f64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumString(val))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<f64>() {
                    Ok(val) => self.visit_f64(val),
                    Err(_) => Ok(NumString(0.0)),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

/// BoolString
///
/// Boolean type holder that can be deserialized from Boolean or String
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(transparent)]
pub struct BoolString(bool);

impl fmt::Display for BoolString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for BoolString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl Visitor<'_> for MyVisitor {
            type Value = BoolString;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("boolean or string")
            }

            fn visit_bool<E>(self, val: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BoolString(val))
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<bool>() {
                    Ok(val) => self.visit_bool(val),
                    Err(_) => Ok(BoolString(false)),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}

/// A reference to a resource by its Name and ID.
#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
pub struct IdAndName {
    /// The name of the entity.
    pub name: String,
    /// The UID for the entity.
    pub id: String,
}

/// A reference to a resource by either its Name or ID.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub enum NameOrId {
    /// Resource ID.
    #[serde(rename = "id")]
    Id(String),
    /// Resource name.
    #[serde(rename = "name")]
    Name(String),
}

#[cfg(test)]
mod tests {
    use serde::de::IntoDeserializer;
    use serde::de::value::{
        BoolDeserializer, Error as ValueError, F64Deserializer, StrDeserializer, U64Deserializer,
    };
    use serde_json::json;

    use super::*;

    #[test]
    fn test_deser_num_str() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestI64 {
            #[serde(deserialize_with = "deser_num_str")]
            pub i: i64,
        }
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestU64 {
            #[serde(deserialize_with = "deser_num_str")]
            pub i: u64,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestF64 {
            #[serde(deserialize_with = "deser_num_str")]
            pub i: f64,
        }

        assert_eq!(
            TestI64 { i: 1 },
            serde_json::from_value(json!({"i": 1})).unwrap()
        );
        assert_eq!(
            TestI64 { i: -1 },
            serde_json::from_value(json!({"i": -1})).unwrap()
        );
        assert_eq!(
            TestU64 { i: 1 },
            serde_json::from_value(json!({"i": 1})).unwrap()
        );
        assert_eq!(
            TestU64 { i: 1 },
            serde_json::from_value(json!({"i": "1"})).unwrap()
        );
        assert_eq!(
            TestU64 { i: 0 },
            serde_json::from_value(json!({"i": ""})).unwrap()
        );
        assert_eq!(
            TestF64 { i: 1.2 },
            serde_json::from_value(json!({"i": 1.2})).unwrap()
        );
        assert_eq!(
            TestF64 { i: -1.2 },
            serde_json::from_value(json!({"i": -1.2})).unwrap()
        );
        assert_eq!(
            TestF64 { i: -1.2 },
            serde_json::from_value(json!({"i": "-1.2"})).unwrap()
        );
        assert_eq!(
            TestF64 { i: 0.0 },
            serde_json::from_value(json!({"i": "0"})).unwrap()
        );
    }
    #[test]
    fn test_deser_num_str_opt() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestI64 {
            #[serde(deserialize_with = "deser_num_str_opt")]
            pub i: Option<i64>,
        }
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestU64 {
            #[serde(deserialize_with = "deser_num_str_opt")]
            pub i: Option<u64>,
        }

        #[derive(Debug, Deserialize, PartialEq)]
        struct TestF64 {
            #[serde(deserialize_with = "deser_num_str_opt")]
            pub i: Option<f64>,
        }

        assert_eq!(
            TestI64 { i: Some(1) },
            serde_json::from_value(json!({"i": 1})).unwrap()
        );

        assert_eq!(
            TestI64 { i: Some(-1) },
            serde_json::from_value(json!({"i": -1})).unwrap()
        );
        assert_eq!(
            TestU64 { i: Some(1) },
            serde_json::from_value(json!({"i": 1})).unwrap()
        );
        assert_eq!(
            TestU64 { i: Some(1) },
            serde_json::from_value(json!({"i": "1"})).unwrap()
        );
        assert_eq!(
            TestU64 { i: Some(0) },
            serde_json::from_value(json!({"i": ""})).unwrap()
        );
        assert_eq!(
            TestF64 { i: Some(1.2) },
            serde_json::from_value(json!({"i": 1.2})).unwrap()
        );
        assert_eq!(
            TestF64 { i: Some(-1.2) },
            serde_json::from_value(json!({"i": -1.2})).unwrap()
        );
        assert_eq!(
            TestF64 { i: Some(-1.2) },
            serde_json::from_value(json!({"i": "-1.2"})).unwrap()
        );
        assert_eq!(
            TestF64 { i: Some(0.0) },
            serde_json::from_value(json!({"i": "0"})).unwrap()
        );
        assert_eq!(
            TestF64 { i: None },
            serde_json::from_value(json!({"i": None::<f64>})).unwrap()
        );
    }

    #[test]
    fn test_deser_bool_str() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            #[serde(deserialize_with = "deser_bool_str")]
            pub i: bool,
        }

        assert_eq!(
            Test { i: false },
            serde_json::from_value(json!({"i": false})).unwrap()
        );
        assert_eq!(
            Test { i: false },
            serde_json::from_value(json!({"i": "false"})).unwrap()
        );
        assert_eq!(
            Test { i: true },
            serde_json::from_value(json!({"i": true})).unwrap()
        );
        assert_eq!(
            Test { i: true },
            serde_json::from_value(json!({"i": "true"})).unwrap()
        );
        assert_eq!(
            Test { i: false },
            serde_json::from_value(json!({"i": "foo"})).unwrap()
        );
        assert_eq!(
            Test { i: false },
            serde_json::from_value(json!({"i": ""})).unwrap()
        );
    }

    #[test]
    fn test_deser_bool_str_opt() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct Test {
            #[serde(deserialize_with = "deser_bool_str_opt")]
            pub i: Option<bool>,
        }
        #[derive(Debug, Deserialize, PartialEq)]
        struct TestDefault {
            #[serde(deserialize_with = "deser_bool_str_opt", default)]
            pub i: Option<bool>,
        }

        assert_eq!(
            Test { i: Some(false) },
            serde_json::from_value(json!({"i": false})).unwrap()
        );
        assert_eq!(
            Test { i: Some(false) },
            serde_json::from_value(json!({"i": "false"})).unwrap()
        );
        assert_eq!(
            Test { i: Some(true) },
            serde_json::from_value(json!({"i": true})).unwrap()
        );
        assert_eq!(
            Test { i: Some(true) },
            serde_json::from_value(json!({"i": "true"})).unwrap()
        );
        assert_eq!(
            Test { i: Some(false) },
            serde_json::from_value(json!({"i": "foo"})).unwrap()
        );
        assert_eq!(
            Test { i: Some(false) },
            serde_json::from_value(json!({"i": ""})).unwrap()
        );
        assert_eq!(
            Test { i: None },
            serde_json::from_value(json!({"i": None::<bool>})).unwrap()
        );
        assert_eq!(
            TestDefault { i: None },
            serde_json::from_value(json!({})).unwrap()
        );
    }

    #[test]
    fn test_deser_intstring_int() {
        let deserializer: U64Deserializer<ValueError> = 1u64.into_deserializer();
        assert_eq!(IntString::deserialize(deserializer), Ok(IntString(1)));
    }

    #[test]
    fn test_intstring_empty_str() {
        let deserializer: StrDeserializer<ValueError> = "".into_deserializer();
        assert_eq!(IntString::deserialize(deserializer), Ok(IntString(0)));
    }

    #[test]
    fn test_intstring_str() {
        let deserializer: StrDeserializer<ValueError> = "5".into_deserializer();
        assert_eq!(IntString::deserialize(deserializer), Ok(IntString(5)));
    }

    #[test]
    fn test_intstring_str_error() {
        let deserializer: StrDeserializer<ValueError> = "foo5".into_deserializer();
        assert_eq!(IntString::deserialize(deserializer), Ok(IntString(0)));
    }

    #[test]
    fn test_intstring_display() {
        assert_eq!(IntString(4).to_string(), "4");
    }

    #[test]
    fn test_numstring_u64() {
        let deserializer: U64Deserializer<ValueError> = 1u64.into_deserializer();
        assert_eq!(NumString::deserialize(deserializer), Ok(NumString(1.0)));
    }

    #[test]
    fn test_numstring_f64() {
        let deserializer: F64Deserializer<ValueError> = 2.3_f64.into_deserializer();
        assert_eq!(NumString::deserialize(deserializer), Ok(NumString(2.3)));
    }

    #[test]
    fn test_numstring_empty_str() {
        let deserializer: StrDeserializer<ValueError> = "".into_deserializer();
        assert_eq!(NumString::deserialize(deserializer), Ok(NumString(0.0)));
    }

    #[test]
    fn test_numstring_str() {
        let deserializer: StrDeserializer<ValueError> = "5".into_deserializer();
        assert_eq!(NumString::deserialize(deserializer), Ok(NumString(5.0)));
    }

    #[test]
    fn test_numstring_str_error() {
        let deserializer: StrDeserializer<ValueError> = "5f".into_deserializer();
        assert_eq!(NumString::deserialize(deserializer), Ok(NumString(0.0)));
    }

    #[test]
    fn test_numstring_display() {
        assert_eq!(NumString(4.1).to_string(), "4.1");
    }

    #[test]
    fn test_boolstring_str() {
        let deserializer: StrDeserializer<ValueError> = "true".into_deserializer();
        assert_eq!(BoolString::deserialize(deserializer), Ok(BoolString(true)));
    }

    #[test]
    fn test_boolstring_str2() {
        let deserializer: StrDeserializer<ValueError> = "false".into_deserializer();
        assert_eq!(BoolString::deserialize(deserializer), Ok(BoolString(false)));
    }

    #[test]
    fn test_boolstring_str_error() {
        let deserializer: StrDeserializer<ValueError> = "foo".into_deserializer();
        assert_eq!(BoolString::deserialize(deserializer), Ok(BoolString(false)));
    }

    #[test]
    fn test_boolstring_bool1() {
        let deserializer: BoolDeserializer<ValueError> = true.into_deserializer();
        assert_eq!(BoolString::deserialize(deserializer), Ok(BoolString(true)));
    }

    #[test]
    fn test_boolstring_bool2() {
        let deserializer: BoolDeserializer<ValueError> = false.into_deserializer();
        assert_eq!(BoolString::deserialize(deserializer), Ok(BoolString(false)));
    }

    #[test]
    fn test_boolstring_display() {
        assert_eq!(BoolString(true).to_string(), "true");
    }
}
