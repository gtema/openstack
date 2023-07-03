// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::{fs::File, ops::Deref};

use chrono::{DateTime, NaiveDate, Utc};

use crate::types::*;

fn datetime(ymd: (i32, u32, u32), time: (u32, u32, u32, u32)) -> DateTime<Utc> {
    NaiveDate::from_ymd_opt(ymd.0, ymd.1, ymd.2)
        .unwrap()
        .and_hms_milli_opt(time.0, time.1, time.2, time.3)
        .unwrap()
        .and_local_timezone(Utc)
        .unwrap()
}
