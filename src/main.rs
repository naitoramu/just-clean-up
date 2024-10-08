use log::debug;
use crate::config::AppConfig;
use crate::server::Server;

mod error;
mod server;
mod config;
mod api;
mod database;
mod domain;
mod middleware;
mod router;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    debug!("Starting server");
    Server::run(
        AppConfig::get().port,
        AppConfig::get().base_path.clone(),
    ).await;
}

#[cfg(test)]
mod tests;
mod context;