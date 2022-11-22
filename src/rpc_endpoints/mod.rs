pub mod account_service_endpoints;
pub mod health_check;
pub mod signup;

use crate::storage_ops::storage_manager::StorageManager;

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

pub struct EndpointServer {
    storage_manager: StorageManager,
}

impl EndpointServer {
    pub async fn new() -> EndpointServer {
        let storage_mgr = StorageManager::new().await;

        EndpointServer {
            storage_manager: storage_mgr,
        }
    }
}
