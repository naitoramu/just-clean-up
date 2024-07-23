use log::debug;
use crate::config::AppConfig;
use crate::server::Server;

mod entities;
mod repositories;
mod error;
mod server;
mod config;
mod mapper;
mod api;
mod database;
mod domain;
mod jwt;

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