use crate::utils::ResourceKey;
use openstack_types::block_storage::v3::backup::response::list_detailed::BackupResponse;
use openstack_types::block_storage::v3::snapshot::response::list_detailed::SnapshotResponse;
use openstack_types::compute::v2::aggregate::response::list_241::AggregateResponse;
use openstack_types::compute::v2::hypervisor::response::list_detailed_253::HypervisorResponse;
use openstack_types::compute::v2::server::response::list_detailed_21::ServerResponse;

impl ResourceKey for ServerResponse {
    fn get_key() -> &'static str {
        "compute.server"
    }
}

impl ResourceKey for AggregateResponse {
    fn get_key() -> &'static str {
        "compute.aggregate"
    }
}

impl ResourceKey for HypervisorResponse {
    fn get_key() -> &'static str {
        "compute.hypervisor"
    }
}

impl ResourceKey for BackupResponse {
    fn get_key() -> &'static str {
        "block_storage.backup"
    }
}

impl ResourceKey for SnapshotResponse {
    fn get_key() -> &'static str {
        "block_storage.snapshot"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::ResourceKey;

    #[test]
    fn server_response_key() {
        assert_eq!(ServerResponse::get_key(), "compute.server");
    }

    #[test]
    fn aggregate_response_key() {
        assert_eq!(AggregateResponse::get_key(), "compute.aggregate");
    }

    #[test]
    fn hypervisor_response_key() {
        assert_eq!(HypervisorResponse::get_key(), "compute.hypervisor");
    }

    #[test]
    fn backup_response_key() {
        assert_eq!(BackupResponse::get_key(), "block_storage.backup");
    }

    #[test]
    fn snapshot_response_key() {
        assert_eq!(SnapshotResponse::get_key(), "block_storage.snapshot");
    }
}
