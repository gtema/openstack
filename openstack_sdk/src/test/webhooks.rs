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

use chrono::{NaiveDate, TimeZone, Utc};
use serde_json::from_str;

use crate::webhooks::*;
use std::fs::File;
use std::io::BufReader;

#[test]
fn test_hookdate_deserialize() {
    let hook: HookDate = from_str("\"2019-01-20 15:00:12 UTC\"").unwrap();
    assert_eq!(
        *hook.as_ref(),
        Utc.with_ymd_and_hms(2019, 1, 20, 15, 00, 12).unwrap(),
    );
    let hook: HookDate = from_str("\"2019-03-01T19:39:17Z\"").unwrap();
    assert_eq!(
        *hook.as_ref(),
        Utc.with_ymd_and_hms(2019, 3, 1, 19, 39, 17).unwrap(),
    );
    let hook: HookDate = from_str("\"2019-03-01T17:50:02.036-05:00\"").unwrap();
    assert_eq!(
        *hook.as_ref(),
        NaiveDate::from_ymd_opt(2019, 3, 1)
            .unwrap()
            .and_hms_milli_opt(22, 50, 2, 36)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap(),
    );
}

#[test]
fn test_pipeline_hook() {
    let file = File::open("src/test/examples/pipeline.json").unwrap();
    let reader = BufReader::new(file);
    let pipeline: PipelineHook = serde_json::from_reader(reader).unwrap();
    assert_eq!(pipeline.object_kind, "pipeline");
    assert_eq!(pipeline.user.username, "mr.example");
    assert_eq!(
        pipeline.object_attributes.before_sha,
        "0000000000000000000000000000000000000000"
    );
}
