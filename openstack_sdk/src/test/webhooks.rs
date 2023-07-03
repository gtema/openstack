// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

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
