use tonic_build::compile_protos;

use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Config {
    application_port: i32,
    database: DatabaseConfig,
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseConfig {
    host: String,
    port: i32,
    username: String,
    password: String,
    database_name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    compile_protos("protos/common.proto")?;
    compile_protos("protos/endpoints.proto")?;

    // Read DB_HOST env var and overwrite the config file based on that
    let db_host: Option<&'static str> = option_env!("DB_HOST_ADDR");
    let db_host = db_host.unwrap_or("127.0.0.1").to_string();

    let config_file = std::fs::File::open("configuration.yaml")?;
    let mut config: Config = serde_yaml::from_reader(config_file)?;
    config.database.host = db_host;

    let config_file = std::fs::File::create("configuration.yaml").expect("File should exist");
    serde_yaml::to_writer(&config_file, &config)?;

    // Overwrite the .env file needed by sqlx to contain the IP of the DB server
    let mut env_file = std::fs::File::create(".env")?;
    let database_url = format!(
        r#"DATABASE_URL="postgres://postgres:password@{}:5432/ev_accounts_test""#,
        config.database.host
    );
    env_file.write_all(database_url.as_bytes())?;
    Ok(())
}
