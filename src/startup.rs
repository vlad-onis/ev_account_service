use crate::rpc_endpoints::account_service_endpoints::account_service::account_service_server::AccountServiceServer;
use crate::rpc_endpoints::account_service_endpoints::AccountsEndpointServer;

use tonic::transport::server::Router;
use tonic::transport::Server;

/// Returns the Router that will start the AccountServiceServer.
pub fn build_service_router() -> Router {
    let accounts_endpoint_server = AccountsEndpointServer::new();

    Server::builder().add_service(AccountServiceServer::new(accounts_endpoint_server))
}
