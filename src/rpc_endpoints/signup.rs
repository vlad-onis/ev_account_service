use super::super::storage_ops::storage_manager::StorageManager;
use super::account_service_endpoints::account_service::{SignUpRequest, SignUpResponse};
use tonic::{Request, Response, Status};

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

/// Signup grpc request.
///
/// params:
/// * SignupRequest, defined in the protofile represents the mandatory request body
///     * username: String - mandatory
///     * password: String - mandatory
///     * email: String - mandatory
// TODO: This should not be public.
pub async fn sign_up(
    request: Request<SignUpRequest>,
    storage_manager: &StorageManager,
) -> Result<Response<SignUpResponse>, Status> {
    storage_manager.get_all_accounts().await;
    let _ = storage_manager.get_account_by_username("vladonis").await;

    let response = SignUpResponse {
        signup_response: request.get_ref().username.clone(),
    };
    Ok(Response::new(response))
}
