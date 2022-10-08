use tonic::{transport::Server, Request, Response, Status};

use account_service::account_service_server::{AccountService, AccountServiceServer};
use account_service::{EmptyRequest, GrpcResponse};

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

#[derive(Debug, Default)]
pub struct EndpointServer {}

#[tonic::async_trait]
impl AccountService for EndpointServer {
    async fn health_check(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<GrpcResponse>, Status> {
        let response = GrpcResponse {
            response_text: "Health check status: healthy".to_string(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let accounts_endpoint_server = EndpointServer::default();

    Server::builder()
        .add_service(AccountServiceServer::new(accounts_endpoint_server))
        .serve(addr)
        .await?;

    Ok(())
}
