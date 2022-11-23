pub mod account_service_endpoints;
pub mod health_check;
pub mod signup;

use crate::storage_ops::storage_manager::StorageManager;

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

pub struct AccountsEndpointServer {
    /// storage_manager handles db operations on user accounts.
    storage_manager: StorageManager,
}

impl AccountsEndpointServer {
    pub async fn new() -> AccountsEndpointServer {
        let storage_mgr = StorageManager::new().await;

        AccountsEndpointServer {
            storage_manager: storage_mgr,
        }
    }
}
