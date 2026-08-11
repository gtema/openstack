#![no_main]

//! Example Guest ABI v1 plugin, used as a test fixture by
//! `openstack-sdk-plugin-wasm`'s integration tests. It implements a toy
//! `v3exampleauth` method: a username/password exchange against
//! `POST /v3/auth/tokens` via the host-provided `identity_http_request`
//! import, structured like a (much simplified) Keystone password auth.

use extism_pdk::*;
use serde_json::{Value, json};

#[host_fn]
extern "ExtismHost" {
    fn identity_http_request(request: String) -> String;
}

#[plugin_fn]
pub fn plugin_abi_version(_input: String) -> FnResult<String> {
    Ok("1".to_string())
}

#[plugin_fn]
pub fn auth_supported_methods(_input: String) -> FnResult<String> {
    Ok(json!(["v3exampleauth"]).to_string())
}

#[plugin_fn]
pub fn auth_api_version(_input: String) -> FnResult<String> {
    Ok(json!([3, 0]).to_string())
}

#[plugin_fn]
pub fn auth_requirements(_input: String) -> FnResult<String> {
    Ok(json!({
        "type": "object",
        "required": ["username", "password"],
        "properties": {
            "username": {"type": "string"},
            "password": {"type": "string"}
        }
    })
    .to_string())
}

#[plugin_fn]
pub fn auth(input: String) -> FnResult<String> {
    let request: Value = serde_json::from_str(&input)?;
    let values = request
        .get("values")
        .and_then(|v| v.as_object())
        .cloned()
        .unwrap_or_default();
    let username = values
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    let password = values
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or_default();

    if username.is_empty() || password.is_empty() {
        return Ok(json!({"error": "username and password are required"}).to_string());
    }

    let body = json!({
        "auth": {
            "identity": {
                "methods": ["password"],
                "password": {
                    "user": {"name": username, "password": password}
                }
            }
        }
    })
    .to_string();

    let http_request = json!({
        "method": "POST",
        "path": "/v3/auth/tokens",
        "headers": {"Content-Type": "application/json"},
        "body": body,
    })
    .to_string();

    let response_json = unsafe { identity_http_request(http_request)? };
    let response: Value = serde_json::from_str(&response_json)?;

    let status = response.get("status").and_then(|v| v.as_u64()).unwrap_or(0);
    if !(200..300).contains(&status) {
        return Ok(json!({"error": format!("identity endpoint returned status {status}")}).to_string());
    }

    let token = response
        .get("headers")
        .and_then(|h| h.get("x-subject-token"))
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    if token.is_empty() {
        return Ok(json!({"error": "identity response did not include a token"}).to_string());
    }

    let auth_info: Value = response
        .get("body")
        .and_then(|b| b.as_str())
        .and_then(|s| serde_json::from_str(s).ok())
        .unwrap_or(Value::Null);

    Ok(json!({"ok": {"token": token, "auth_info": auth_info}}).to_string())
}
