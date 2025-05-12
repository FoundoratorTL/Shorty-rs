use anyhow::Result;
use dotenv::dotenv;
use std::{env, net::SocketAddr};

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub base_url: String,
    pub server_addr: SocketAddr,
}

impl Config {
    /// Load from environment, returning `anyhow::Error` on failure.
    pub fn from_env() -> Result<Self> {
        dotenv().ok(); // load .env into env
        let database_url = env::var("DATABASE_URL")?;  // ? yields anyhow::Error
        let base_url     = env::var("BASE_URL")?;
        let server_addr  = env::var("SERVER_ADDR")?.parse()?;

        Ok(Self { database_url, base_url, server_addr })
    }
}
