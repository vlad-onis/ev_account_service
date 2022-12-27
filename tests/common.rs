use ev_account_service::configuration::get_configuration;
use ev_account_service::startup;

use std::net::AddrParseError;
use std::net::SocketAddr;

/// Function used to spawn an endpoint server that will listen for incoming grpc requests
/// Every test will act as a grpc client.
// TODO: Use this file to initialize app in tests.
pub async fn spawn_endpoint_server() -> Result<(), AddrParseError> {
    let config = get_configuration().expect("Failed to read configuration");

    let sv = startup::build_service_router();

    let addr = format!("127.0.0.1:{}", config.application_port);
    let addr: SocketAddr = addr.parse()?;

    tokio::spawn(async move {
        _ = sv.serve(addr).await;
    });

    Ok(())
}
