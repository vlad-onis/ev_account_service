use tonic::{Request, Response, Status};

use super::account_service_endpoints::account_service::{EmptyRequest, GrpcResponse};

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

/// Grpc request will return the health check response

// TODO: This should not be public.
pub async fn health_check_handler(
    _request: Request<EmptyRequest>,
) -> Result<Response<GrpcResponse>, Status> {
    let response = GrpcResponse {
        response_text: "Health check status: healthy".to_string(),
    };
    Ok(Response::new(response))
}
