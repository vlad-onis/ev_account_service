mod rpc_endpoints;

use rpc_endpoints::health_check::account_service::account_service_server::AccountServiceServer;
use rpc_endpoints::EndpointServer;

use tonic::transport::Server;

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
