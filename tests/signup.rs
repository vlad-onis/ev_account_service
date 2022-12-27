use ev_account_service::configuration::get_configuration;
use ev_account_service::rpc_endpoints::health_check::account_service::account_service_client::AccountServiceClient;
use ev_account_service::rpc_endpoints::health_check::account_service::SignUpRequest;
use ev_account_service::startup;
use sqlx::Connection;

use sqlx::PgConnection;
use std::net::AddrParseError;
use std::net::SocketAddr;
use tonic;

use tokio::time::{sleep, Duration};

async fn spawn_endpoint_server() -> Result<(), AddrParseError> {
    let config = get_configuration().expect("Failed to read configuration");

    let sv = startup::build_service_router();

    let addr = format!("127.0.0.1:{}", config.application_port);
    let addr: SocketAddr = addr.parse()?;

    tokio::spawn(async move {
        _ = sv.serve(addr).await;
    });

    Ok(())
}

#[tokio::test]
async fn signup_returns_registered_user() {
    let app_started = spawn_endpoint_server().await;
    sleep(Duration::from_millis(50)).await; //Waiting for the Endpoint server to actually stary
    assert!(app_started.is_ok());

    let config = get_configuration();
    assert!(config.is_ok());
    let config = config.unwrap();

    let addr = format!("http://127.0.0.1:{}", config.application_port);
    let client = AccountServiceClient::connect(addr).await;
    assert!(client.is_ok());

    let connection_string = config.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to the db");

    let mut client = client.unwrap();

    let request = tonic::Request::new(SignUpRequest {
        username: String::from("vladonis"),
        password: String::from("test1234"),
        email: String::from("vladonis@gmail.com"),
    });

    // TODO: Needs to typed not stringy typed
    let response = client.sign_up(request).await;
    assert!(response.is_ok());
    let response = response.unwrap();
    let expected = format!("User: vladonis signed up successfully!");
    assert_eq!(response.get_ref().signup_response, expected);

    let _ = sqlx::query!("DELETE FROM accounts where username='vladonis'",)
        .execute(&mut connection)
        .await
        .expect("Could not fetch signed up user");
}
