use crate::rpc_endpoints::health_check::account_service::account_service_server::AccountServiceServer;
use crate::rpc_endpoints::EndpointServer;

use tonic::transport::server::Router;
use tonic::transport::Server;

pub fn run() -> Router {
    let accounts_endpoint_server = EndpointServer::default();

    Server::builder().add_service(AccountServiceServer::new(accounts_endpoint_server))
}
