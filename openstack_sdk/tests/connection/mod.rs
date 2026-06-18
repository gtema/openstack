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

//! All connection tests share the same auth cache file on disk (same OS_CLOUD
//! config). They are serialized via the `serial-connection` test group in
//! `.config/nextest.toml` (nextest only). Running with regular `cargo test`
//! assumes the caller handles serialization.

#[cfg(feature = "async")]
mod r#async;
#[cfg(all(feature = "async", feature = "sync", feature = "identity"))]
mod catalog;
#[cfg(all(feature = "async", feature = "sync", feature = "identity"))]
mod reauth;
#[cfg(feature = "sync")]
mod sync;
