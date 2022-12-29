use super::super::storage_ops::storage_manager::StorageManager;
use super::account_service_endpoints::account_service::{SignUpRequest, SignUpResponse};
use crate::model::account::Account;

use anyhow::Result;
use tonic::{Request, Response};

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
pub async fn sign_up_handler(
    request: Request<SignUpRequest>,
    storage_manager: &StorageManager,
) -> Result<Response<SignUpResponse>> {
    let account = Account::new(
        request.get_ref().username.clone(),
        request.get_ref().email.clone(),
        request.get_ref().password.clone(),
    )?;

    let inserted = storage_manager.insert_account(account.clone()).await;

    let response = match inserted {
        Ok(account) => SignUpResponse {
            signup_response: format!("User: {} signed up successfully!", account.username),
        },
        Err(er) => {
            tracing::error!("Signup error: {}", er);
            SignUpResponse {
                signup_response: format!("Failed to sign up user: {} ", er),
            }
        }
    };

    Ok(Response::new(response))
}
