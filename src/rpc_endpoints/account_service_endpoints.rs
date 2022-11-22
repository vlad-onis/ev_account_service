use super::account_service::account_service_server::AccountService;
use super::account_service::{EmptyRequest, GrpcResponse, SignUpRequest, SignUpResponse};
use super::health_check::health_check;
use super::signup::sign_up;
use super::EndpointServer;

use tonic::{Request, Response, Status};

#[tonic::async_trait]
impl AccountService for EndpointServer {
    async fn health_check(
        &self,
        request: Request<EmptyRequest>,
    ) -> Result<Response<GrpcResponse>, Status> {
        health_check(request).await
    }

    async fn sign_up(
        &self,
        request: Request<SignUpRequest>,
    ) -> Result<Response<SignUpResponse>, Status> {
        sign_up(request, &self.storage_manager).await
    }
}
