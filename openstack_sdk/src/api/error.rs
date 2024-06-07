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

use std::any;
use std::error::Error;

use http::Uri;
use thiserror::Error;

use crate::api::PaginationError;

/// Errors which may occur when creating form data.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BodyError {
    /// Body data could not be serialized from form parameters.
    #[error("failed to URL encode form parameters: {}", source)]
    UrlEncoded {
        /// The source of the error.
        #[from]
        source: serde_urlencoded::ser::Error,
    },
    #[error("failed to serialize request body: {}", source)]
    Serialize {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}

/// Errors which may occur when using API endpoints.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    /// The URL failed to parse.
    #[error("failed to parse url: {}", source)]
    UrlParse {
        /// The source of the error.
        #[from]
        source: url::ParseError,
    },
    /// Body data could not be created.
    #[error("failed to create form data: {}", source)]
    Body {
        /// The source of the error.
        #[from]
        source: BodyError,
    },
    /// JSON deserialization from OpenStack failed.
    #[error("could not parse JSON response: {}", source)]
    Json {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    /// Resource can not be found using known locations.
    #[error("resource cannot be found")]
    ResourceNotFound,
    /// Too many candidates to identitfy resource by identifier
    #[error("cannot uniqly find resource by identifier")]
    IdNotUnique,
    /// OpenStack session error.
    #[error("openstack session error: {}", msg)]
    Session {
        /// The error message from OpenStack.
        msg: String,
    },
    /// OpenStack returned understandable error message
    #[error(
        "openstack server error:\n\turi: `{}`\n\tstatus: `{}`\n\tmessage: `{}`",
        uri,
        status,
        msg
    )]
    OpenStack {
        /// The status code for the return.
        status: http::StatusCode,
        /// The URI of the request
        uri: Uri,
        /// The error message from OpenStack.
        msg: String,
    },
    /// OpenStack returned an error without JSON information.
    #[error(
        "openstack internal server error:\n\turi: `{}`\n\tstatus: `{}`",
        uri,
        status
    )]
    OpenStackService {
        /// The status code for the return.
        status: http::StatusCode,
        /// The URI of the request
        uri: Uri,
        /// The error data from OpenStack.
        data: String,
    },
    /// OpenStack returned an HTTP error with JSON we did not recognize.
    #[error("openstack server error: {:?}", obj)]
    OpenStackUnrecognized {
        /// The status code for the return.
        status: http::StatusCode,
        /// The URI of the request
        uri: Uri,
        /// The full object from OpenStack.
        obj: serde_json::Value,
    },
    /// Failed to parse an expected data type from JSON.
    #[error("could not parse {} data from JSON: {}", typename, source)]
    DataType {
        /// The source of the error.
        source: serde_json::Error,
        /// The name of the type that could not be deserialized.
        typename: &'static str,
    },
    /// An error with pagination occurred.
    #[error("failed to handle for pagination: {}", source)]
    Pagination {
        /// The source of the error.
        #[from]
        source: PaginationError,
    },
    #[error("failed to handle for catalog: {}", source)]
    Catalog {
        /// The source of the error.
        #[from]
        source: crate::catalog::CatalogError,
    },
}

impl<E> ApiError<E>
where
    E: Error + Send + Sync + 'static,
{
    /// Create an API error in a client error.
    pub fn client(source: E) -> Self {
        ApiError::Client { source }
    }
    /// Create an API error in a catalog error.
    pub fn catalog(source: crate::catalog::CatalogError) -> Self {
        ApiError::Catalog { source }
    }

    /// Process server response with no Json body
    pub(crate) fn server_error(
        uri: Option<Uri>,
        status: http::StatusCode,
        body: &bytes::Bytes,
    ) -> Self {
        // Non Json body response ends in this function
        if http::StatusCode::NOT_FOUND.as_u16() == status {
            return Self::OpenStack {
                status,
                uri: uri.unwrap_or(Uri::from_static("/")),
                msg: String::new(),
            };
        };

        Self::OpenStackService {
            status,
            uri: uri.unwrap_or(Uri::from_static("/")),
            data: String::from_utf8_lossy(body).into(),
        }
    }

    /// Process server error response with Json body
    pub(crate) fn from_openstack(
        uri: Option<Uri>,
        status: http::StatusCode,
        value: serde_json::Value,
    ) -> Self {
        if http::StatusCode::NOT_FOUND.as_u16() == status {
            return Self::OpenStack {
                status,
                uri: uri.unwrap_or(Uri::from_static("/")),
                msg: value.to_string(),
            };
        };

        let error_value = value
            .pointer("/message")
            .or_else(|| value.pointer("/error"));

        if let Some(error_value) = error_value {
            if let Some(msg) = error_value.as_str() {
                // Error we know how to parse
                ApiError::OpenStack {
                    status,
                    uri: uri.unwrap_or(Uri::from_static("/")),
                    msg: msg.into(),
                }
            } else {
                // Error we do not know how to parse
                ApiError::OpenStackUnrecognized {
                    status,
                    uri: uri.unwrap_or(Uri::from_static("/")),
                    obj: error_value.clone(),
                }
            }
        } else {
            ApiError::OpenStackUnrecognized {
                status,
                uri: uri.unwrap_or(Uri::from_static("/")),
                obj: value,
            }
        }
    }

    pub(crate) fn data_type<T>(source: serde_json::Error) -> Self {
        ApiError::DataType {
            source,
            typename: any::type_name::<T>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use http::Uri;
    use serde_json::json;
    use thiserror::Error;

    use crate::api::ApiError;

    #[derive(Debug, Error)]
    #[error("my error")]
    enum MyError {}

    #[test]
    fn openstack_error_error() {
        let obj = json!({
            "error": "error contents",
        });

        let err: ApiError<MyError> = ApiError::from_openstack(
            Some(Uri::from_static("http://foo.bar")),
            http::StatusCode::CONFLICT,
            obj.clone(),
        );
        if let ApiError::OpenStack { status, uri, msg } = err {
            assert_eq!(uri, Uri::from_static("http://foo.bar"));
            assert_eq!(msg, "error contents");
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn openstack_error_message_string() {
        let obj = json!({
            "message": "error contents",
        });

        let err: ApiError<MyError> = ApiError::from_openstack(
            Some(Uri::from_static("http://foo.bar")),
            http::StatusCode::CONFLICT,
            obj.clone(),
        );
        if let ApiError::OpenStack { status, uri, msg } = err {
            assert_eq!(uri, Uri::from_static("http://foo.bar"));
            assert_eq!(msg, "error contents");
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn openstack_error_message_object() {
        let err_obj = json!({
            "blah": "foo",
        });
        let obj = json!({
            "message": err_obj,
        });

        let err: ApiError<MyError> = ApiError::from_openstack(
            Some(Uri::from_static("http://foo.bar")),
            http::StatusCode::CONFLICT,
            obj.clone(),
        );
        if let ApiError::OpenStackUnrecognized { status, uri, obj } = err {
            assert_eq!(uri, Uri::from_static("http://foo.bar"));
            assert_eq!(obj, err_obj);
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn openstack_error_message_unrecognized() {
        let err_obj = json!({
            "some_weird_key": "an even weirder value",
        });

        let err: ApiError<MyError> = ApiError::from_openstack(
            Some(Uri::from_static("http://foo.bar")),
            http::StatusCode::CONFLICT,
            err_obj.clone(),
        );
        if let ApiError::OpenStackUnrecognized { status, uri, obj } = err {
            assert_eq!(uri, Uri::from_static("http://foo.bar"));
            assert_eq!(obj, err_obj);
            assert_eq!(status, http::StatusCode::CONFLICT);
        } else {
            panic!("unexpected error: {}", err);
        }
    }

    #[test]
    fn openstack_error_not_found() {
        let err_obj = json!({
            "some_weird_key": "an even weirder value",
        });

        let err: ApiError<MyError> = ApiError::from_openstack(
            Some(Uri::from_static("http://foo.bar")),
            http::StatusCode::NOT_FOUND,
            err_obj.clone(),
        );
        if let ApiError::OpenStack { status, uri, msg } = err {
            assert_eq!(uri, Uri::from_static("http://foo.bar"));
            assert_eq!(msg, err_obj.to_string());
            assert_eq!(status, http::StatusCode::NOT_FOUND);
        } else {
            panic!("unexpected error: {}", err);
        }
    }
}
