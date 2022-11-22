use crate::rpc_endpoints::account_service::account_service_server::AccountServiceServer;
use crate::rpc_endpoints::EndpointServer;

use tonic::transport::server::Router;
use tonic::transport::Server;

pub async fn run() -> Router {
    let accounts_endpoint_server = EndpointServer::new().await;

    Server::builder().add_service(AccountServiceServer::new(accounts_endpoint_server))
}
