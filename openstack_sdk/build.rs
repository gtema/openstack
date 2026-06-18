#![allow(clippy::expect_used, clippy::unwrap_used)]

//! Automatically anchors auth plugin crates from Cargo.toml.
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let manifest =
        fs::read_to_string(env::var_os("CARGO_MANIFEST").unwrap_or_else(|| "Cargo.toml".into()))
            .expect("cannot read Cargo.toml");
    let feature_map = scan_features(&manifest);
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let dest = std::path::Path::new(&out_dir).join("plugin_anchors.rs");
    let mut in_deps = false;
    let mut lines = Vec::new();

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_deps = trimmed == "[dependencies]";
            continue;
        }
        if !in_deps {
            continue;
        }

        let Some((raw_key, rest)) = trimmed.split_once('=') else {
            continue;
        };
        let raw_key = raw_key.trim();
        let dep_name = if let Some((b, _)) = raw_key.split_once('.') {
            b.to_string()
        } else {
            raw_key.to_string()
        };

        if dep_name == "openstack-sdk-auth-core" || !dep_name.starts_with("openstack-sdk-auth-") {
            continue;
        }

        let rust_name = dep_name.replace('-', "_");
        let dep_suffix = &dep_name["openstack-sdk-auth-".len()..];

        if rest.contains("optional") {
            if let Some(feat) = feature_map.get(&dep_name) {
                lines.push(format!(
                    "#[cfg(feature = \"{}\")]\nuse {}::ANCHOR as ANCHOR_{};",
                    feat, rust_name, dep_suffix
                ));
            } else {
                // No feature found - treat as non-optional
                lines.push(format!(
                    "use {}::ANCHOR as ANCHOR_{};",
                    rust_name, dep_suffix
                ));
            }
        } else {
            lines.push(format!(
                "use {}::ANCHOR as ANCHOR_{};",
                rust_name, dep_suffix
            ));
        }
    }

    fs::write(&dest, lines.join("\n\n")).expect("cannot write plugin_anchors.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

/// Maps dep_suffix (e.g., "federation") → feature_name
fn scan_features(manifest: &str) -> HashMap<String, String> {
    let mut in_features = false;
    let mut current_feature: Option<String> = None;
    let mut result = HashMap::new();

    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed == "[features]" {
            in_features = true;
            current_feature = None;
            continue;
        }
        if !in_features {
            continue;
        }
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            break;
        }

        if let Some((feat_name, rest)) = trimmed.split_once('=') {
            current_feature = Some(feat_name.trim().to_string());
            if let Some(feat) = &current_feature {
                for part in rest.split([',', ';', '[', ']']) {
                    let part = part.trim().trim_matches('"');
                    if let Some(suffix) = part.strip_prefix("dep:") {
                        result.insert(suffix.to_string(), feat.clone());
                    }
                }
            }
        } else if let Some(feat) = &current_feature {
            for part in trimmed.split([',', ';', '[', ']']) {
                let part = part.trim().trim_matches('"');
                if let Some(suffix) = part.trim().strip_prefix("dep:") {
                    result.insert(suffix.to_string(), feat.clone());
                }
            }
        }
    }
    result
}
