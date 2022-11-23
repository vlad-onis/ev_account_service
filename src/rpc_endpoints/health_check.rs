use tonic::{Request, Response, Status};

use super::account_service::{EmptyRequest, GrpcResponse};

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

pub async fn health_check(
    _request: Request<EmptyRequest>,
) -> Result<Response<GrpcResponse>, Status> {
    let response = GrpcResponse {
        response_text: "Health check status: healthy".to_string(),
    };
    Ok(Response::new(response))
}
