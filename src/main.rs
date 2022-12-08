use ev_account_service::configuration::get_configuration;
use ev_account_service::startup;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = get_configuration().expect("Failed to read configuration");

    let router = startup::run();
    let addr = format!("127.0.0.1:{}", configuration.application_port);
    let addr: SocketAddr = addr.parse()?;

    router.serve(addr).await?;
    Ok(())
}
