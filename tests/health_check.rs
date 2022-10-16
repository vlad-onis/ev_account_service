use ev_account_service::rpc_endpoints::health_check::account_service::account_service_client::AccountServiceClient;
use ev_account_service::rpc_endpoints::health_check::account_service::EmptyRequest;
use ev_account_service::startup;

use std::net::AddrParseError;
use std::net::SocketAddr;
use tonic;

use tokio::time::{sleep, Duration};

async fn spawn_endpoint_server() -> Result<(), AddrParseError> {
    let sv = startup::run();

    let addr: SocketAddr = "[::1]:50051".parse()?;

    tokio::spawn(async move {
        _ = sv.serve(addr).await;
    });

    Ok(())
}

#[tokio::test]
async fn health_check_works() {
    let app_started = spawn_endpoint_server().await;
    sleep(Duration::from_millis(50)).await; //Waiting for the Endpoint server to actually stary
    assert!(app_started.is_ok());

    let client = AccountServiceClient::connect("http://[::1]:50051").await;
    assert!(client.is_ok());

    let mut client = client.unwrap();
    let request = tonic::Request::new(EmptyRequest {});
    let response = client.health_check(request).await;

    assert!(response.is_ok());

    let response = response.unwrap();
    assert_eq!(
        response.get_ref().response_text,
        "Health check status: healthy"
    );
}
