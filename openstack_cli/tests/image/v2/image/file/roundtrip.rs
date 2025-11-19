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

use assert_cmd::prelude::*;
use futures::StreamExt;
use md5::Context;
use rand::distr::{Alphanumeric, SampleString};
use reqwest::{Client, header};
use serde_json::Value;
use std::process::Command;
use std::{error::Error, path::PathBuf};
use tempfile::{Builder, TempDir};
use tokio::{fs::File, io::AsyncReadExt, io::AsyncWriteExt};

/// Downloads a file, saving it with the filename provided by the server.
/// Returns (final_path, md5_checksum).
pub async fn download_with_md5_and_filename(
    url: &str,
    tmp_dir: &TempDir,
) -> Result<(PathBuf, String), Box<dyn Error>> {
    let retries = reqwest::retry::for_host("download.cirros-cloud.net")
        .max_retries_per_request(3)
        .classify_fn(|req_rep| {
            if req_rep.status() == Some(http::StatusCode::FORBIDDEN) {
                req_rep.retryable()
            } else {
                req_rep.success()
            }
        });
    let client = Client::builder().retry(retries).gzip(true).build()?;
    let response = client.get(url).send().await?;
    response.error_for_status_ref()?; // fail fast on HTTP errors

    // Try to extract filename from Content-Disposition or fallback to URL
    let filename = response
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .and_then(|val| val.to_str().ok())
        .and_then(parse_filename_from_content_disposition)
        .or_else(|| extract_filename_from_url(url))
        .unwrap_or_else(|| "download.bin".to_string());

    let path = tmp_dir.path().join(&filename);
    let mut file = File::create(&path).await?;
    let mut context = Context::new();

    let mut stream = response.bytes_stream();
    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
        context.consume(&chunk);
    }

    file.flush().await?;
    let digest = context.compute();
    let checksum = format!("{:x}", digest);

    Ok((path, checksum))
}

/// Parse filename from a Content-Disposition header value.
fn parse_filename_from_content_disposition(header_value: &str) -> Option<String> {
    // Simple extraction for headers like: attachment; filename="example.txt"
    header_value.split(';').find_map(|part| {
        let part = part.trim();
        if part.starts_with("filename=") {
            Some(
                part.trim_start_matches("filename=")
                    .trim_matches('"')
                    .to_string(),
            )
        } else {
            None
        }
    })
}

/// Extracts filename from the URL path (fallback)
fn extract_filename_from_url(url: &str) -> Option<String> {
    url.split('/')
        .filter(|s| !s.is_empty())
        .last()
        .map(|s| s.to_string())
}

#[tokio::test]
async fn image_upload_download_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = Builder::new().prefix("data").tempdir()?;
    let cirros_ver = "0.6.3";
    // let target = format!(
    //     "http://download.cirros-cloud.net/{ver}/cirros-{ver}-x86_64-disk.img",
    //     ver = cirros_ver
    // );
    let target = "https://github.com/gtema/openstack/releases/download/internal/cirros-0.6.3-x86_64-disk.img";
    let (fname, checksum) = download_with_md5_and_filename(&target, &tmp_dir)
        .await
        .expect("Download failed");
    assert_eq!(
        "87617e24a5e30cb3b87fda8c0764838f", checksum,
        "Download checksum matches the expected"
    );

    let img_name = format!(
        "test-rust-{}",
        Alphanumeric.sample_string(&mut rand::rng(), 16)
    );

    // Create image
    let output = Command::cargo_bin("osc")?
        .arg("image")
        .arg("image")
        .arg("create")
        .arg("--name")
        .arg(img_name.clone())
        .arg("--container-format")
        .arg("bare")
        .arg("--disk-format")
        .arg("qcow2")
        .arg("-o")
        .arg("json")
        .output()
        .expect("failed to execute process");
    let data: Value = serde_json::from_slice(&output.stdout)?;
    let image_id = data["id"].as_str().expect("ID is present");

    // Upload data
    Command::cargo_bin("osc")?
        .arg("image")
        .arg("image")
        .arg("upload")
        .arg(image_id)
        .arg("--file")
        .arg(fname.clone())
        .assert()
        .success();

    // Download image
    let download_data_fname = tmp_dir.path().join("download_data");
    Command::cargo_bin("osc")?
        .arg("image")
        .arg("image")
        .arg("download")
        .arg(image_id)
        .arg("--file")
        .arg(download_data_fname.clone())
        .assert()
        .success();

    // Compare files
    let mut file = File::open(download_data_fname).await?;
    let mut buffer = [0u8; 8192];
    let mut context = Context::new();

    loop {
        let n = file.read(&mut buffer).await?;
        if n == 0 {
            break;
        }
        context.consume(&buffer[..n]);
    }
    let download_digest = context.compute();

    // Delete image
    Command::cargo_bin("osc")?
        .arg("image")
        .arg("image")
        .arg("delete")
        .arg(image_id)
        .assert()
        .success();

    assert_eq!(
        format!("{:x}", download_digest),
        checksum,
        "Checksums match"
    );

    Ok(())
}
