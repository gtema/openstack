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

//! Results pagination

use thiserror::Error;

use crate::api::paged::next_page::LinkHeaderParseError;

/// Errors which may occur with pagination.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PaginationError {
    /// No pagination information available in the body.
    #[error("failed to find pagination in the response: {}", msg)]
    Body { msg: String },
    /// An invalid URL.
    #[error("failed to parse a Link HTTP URL: {} {}", url, source)]
    InvalidUrl {
        url: String,
        /// The source of the error.
        #[source]
        source: url::ParseError,
    },
    /// A `Link` HTTP header cannot be parsed.
    #[error("failed to parse a Link HTTP header: {}", source)]
    LinkHeader {
        /// The source of the error.
        #[from]
        source: LinkHeaderParseError,
    },
}

/// Pagination options
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Pagination {
    /// Return all results.
    #[default]
    All,
    /// Limit to a total number of results.
    Limit(usize),
}

impl Pagination {
    pub(crate) fn page_limit(self) -> usize {
        usize::MAX
    }

    pub(crate) fn is_last_page(self, last_page_size: usize, num_results: usize) -> bool {
        // If the last page has fewer elements than our limit, we're definitely done.
        if self.page_limit() < usize::MAX && last_page_size < self.page_limit() {
            return true;
        }

        // If last page is empty we are definitely done
        if last_page_size == 0 {
            return true;
        }

        // Otherwise, check if we have results which fill our limit.
        if let Pagination::Limit(limit) = self {
            return limit <= num_results;
        }

        // We're not done yet.
        false
    }
}

/// A query modifier that paginates an endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paged<E> {
    pub(in crate::api::paged) endpoint: E,
    pub(in crate::api::paged) pagination: Pagination,
}

#[cfg(test)]
mod tests {
    use crate::api::Pagination;

    #[test]
    fn pagination_default() {
        assert_eq!(Pagination::default(), Pagination::All);
    }
}
