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

//! Utilities

use std::path::{Path, PathBuf};

/// Expand tilde in the file path
pub(crate) fn expand_tilde<P: AsRef<Path>>(path_user_input: P) -> Option<PathBuf> {
    let path = path_user_input.as_ref();
    if !path.starts_with("~") {
        return Some(path.to_path_buf());
    }
    if path == Path::new("~") {
        return dirs::home_dir();
    }
    dirs::home_dir().map(|mut home| {
        if home == Path::new("/") {
            // Corner case: `home` is root directory;
            // don't prepend extra `/`, just drop the tilde.
            path.strip_prefix("~").unwrap_or(path).to_path_buf()
        } else {
            home.push(path.strip_prefix("~").unwrap_or(path));
            home
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let home = std::env::var("HOME").unwrap();
        assert_eq!(
            expand_tilde("~/dummy").unwrap(),
            PathBuf::from(format!("{home}/dummy"))
        );
        assert_eq!(
            expand_tilde("/root/dummy").unwrap(),
            PathBuf::from("/root/dummy")
        );
    }
}
