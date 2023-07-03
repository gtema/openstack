use indicatif;
use reqwest;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OpenStackCliError {
    #[error("failed to serialize data to json: {}", source)]
    SerializeJson {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
    #[error("failed to serialize data to yaml: {}", source)]
    SerializeYaml {
        /// The source of the error.
        #[from]
        source: serde_yaml::Error,
    },
    #[error("OpenStackSDK error: {}", source)]
    OpenStackSDK {
        /// The source of the error.
        #[from]
        source: openstack_sdk::OpenStackError,
    },
    #[error("OpenStackSDK error: {}", source)]
    OpenStackApi {
        /// The source of the error.
        #[from]
        source: openstack_sdk::api::ApiError<openstack_sdk::RestError>,
    },

    #[error("Config error: {}", source)]
    ConfigError {
        #[from]
        source: openstack_sdk::config::ConfigError,
    },

    #[error("resource not found")]
    ResourceNotFound,

    #[error("cannot uniqly findresource by identifier")]
    IdNotUnique,

    #[error("IO error: {}", source)]
    IO {
        #[from]
        source: std::io::Error,
    },
    #[error("Reqwest error: {}", source)]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("Indicativ error: {}", source)]
    Idinticatif {
        #[from]
        source: indicatif::style::TemplateError,
    },
    #[error("OpenStackSDK endpoint builder error: `{0}`")]
    EndpointBuild(String),

    #[error("Cloud connection for `{0:?}` cannot be found")]
    ConnectionNotFound(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
