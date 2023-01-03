use ev_account_service::configuration::get_configuration;
use ev_account_service::startup;

use std::net::SocketAddr;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let configuration = get_configuration().expect("Failed to read configuration");

    let router = startup::build_service_router();

    // TODO: Make this a config instead of a hardcoded local run
    let addr = format!("0.0.0.0:{}", configuration.application_port);
    let addr: SocketAddr = addr.parse()?;

    router.serve(addr).await?;
    Ok(())
}
