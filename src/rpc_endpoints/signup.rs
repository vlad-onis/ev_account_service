use super::super::storage_ops::storage_manager::StorageManager;
use super::account_service::{SignUpRequest, SignUpResponse};

use tonic::{Request, Response, Status};

pub mod account_service {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("account_service_rpc"); // String specified here matches the proto package name
}

pub async fn sign_up(
    request: Request<SignUpRequest>,
    storage_manager: &StorageManager,
) -> Result<Response<SignUpResponse>, Status> {
    println!("Username = {}", request.get_ref().username);
    println!("Password = {}", request.get_ref().password);

    storage_manager.get_all_accounts().await;

    let response = SignUpResponse {
        signup_response: request.get_ref().username.clone(),
    };
    Ok(Response::new(response))
}
