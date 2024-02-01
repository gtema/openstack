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
