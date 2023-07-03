//! Compute API v2 data types
use serde::{Deserialize, Serialize};

/// Flavors
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Flavor {
    /// The name of the flavor.
    pub name: String,
    /// The UID for the Flavor.
    pub id: String,
}

/// Servers
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    /// The name of the Server.
    pub name: String,
    /// The UID for the Server.
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub access_ip_v4: Option<String>,
}
