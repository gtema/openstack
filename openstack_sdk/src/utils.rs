//! Utilities
//!
use serde::{Deserialize, Deserializer};
use serde_json::Value;

/// Try to deserialize data and return `Default` if that fails
pub fn deser_ok_or_default<'a, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'a> + Default,
    D: Deserializer<'a>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    Ok(T::deserialize(v).unwrap_or_default())
}
