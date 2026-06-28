use crate::utils::ResourceKey;
use openstack_types::compute::v2::server::response::list_detailed_21::ServerResponse;

impl ResourceKey for ServerResponse {
    fn get_key() -> &'static str {
        "compute.server"
    }
}
