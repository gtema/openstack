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
use file_diff::diff_files;
use rand::distr::{Alphanumeric, SampleString};
use serde_json::Value;
use std::fs::File;
use std::io::Cursor;
use std::io::copy;
use std::process::Command;
use tempfile::Builder;

#[tokio::test]
async fn image_upload_download_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = Builder::new().prefix("data").tempdir()?;
    let cirros_ver = "0.6.2";
    let target = format!(
        "http://download.cirros-cloud.net/{ver}/cirros-{ver}-x86_64-disk.img",
        ver = cirros_ver
    );
    let response = reqwest::get(target).await?;
    let (mut img_data, fname) = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        let fname = tmp_dir.path().join(fname);
        (File::create(fname.clone())?, fname)
    };
    let mut content = Cursor::new(response.bytes().await?);
    copy(&mut content, &mut img_data)?;

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
    diff_files(
        &mut img_data,
        &mut File::open(&download_data_fname).unwrap(),
    );

    // Delete image
    Command::cargo_bin("osc")?
        .arg("image")
        .arg("image")
        .arg("delete")
        .arg(image_id)
        .assert()
        .success();

    Ok(())
}
