use anyhow;
use thiserror::Error;

/// Errors which may occur with pagination.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum PaginationError {
    #[error("failed to find pagination in the response: {}", msg)]
    Body { msg: String },
    /// An invalid URL can be returned.
    #[error("failed to parse a Link HTTP URL: {} {}", url, source)]
    InvalidUrl {
        url: String,
        /// The source of the error.
        #[source]
        source: url::ParseError,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

/// Pagination options
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Pagination {
    /// Return all results.
    ///
    /// Note that most endpoints may have a server-side limit to the number of results.
    #[default]
    All,
    /// Limit to a number of results.
    Limit(usize),
}

const MAX_PAGE_SIZE: usize = 100;

impl Pagination {
    pub(crate) fn page_limit(self) -> usize {
        match self {
            // Set page size to max
            Pagination::All => MAX_PAGE_SIZE,
            // Set page size to min(limit, max_page_size)
            Pagination::Limit(size) => size.min(MAX_PAGE_SIZE),
        }
    }

    pub(crate) fn is_last_page(self, last_page_size: usize, num_results: usize) -> bool {
        // If the last page has fewer elements than our limit, we're definitely done.
        if last_page_size < self.page_limit() {
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
