use super::health_check::health_check_handler;
use super::signup::sign_up_handler;
use crate::storage_ops::storage_manager::StorageManager;

use account_service::account_service_server::AccountService;
use account_service::{EmptyRequest, GrpcResponse, SignUpRequest, SignUpResponse};

use tonic::{Request, Response, Status};

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

pub struct AccountsEndpointServer {
    /// storage_manager handles db operations on user accounts.
    storage_manager: StorageManager,
}

impl AccountsEndpointServer {
    pub fn new() -> AccountsEndpointServer {
        let storage_mgr = StorageManager::new();

        AccountsEndpointServer {
            storage_manager: storage_mgr,
        }
    }
}

impl Default for AccountsEndpointServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tonic::async_trait]
impl AccountService for AccountsEndpointServer {
    /// Healthcheck grpc endpoint
    async fn health_check(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<GrpcResponse>, Status> {
        health_check_handler(request).await
    }

    /// Signup grpc request.   
    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        let res = sign_up_handler(request, &self.storage_manager)
            .await
            .map_err(|e| {
                let error = format!("Signup failed: {}", e);
                Status::aborted(error)
            })?;

        Ok(res)
    }
}
