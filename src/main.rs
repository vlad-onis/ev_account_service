use ev_account_service::startup;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = startup::run();

    let addr: SocketAddr = "[::1]:50051".parse()?;
    router.serve(addr).await?;

    Ok(())
}
