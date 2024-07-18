use crate::database::database::Database;
use crate::server::Server;

mod entities;
mod repositories;
mod error;
mod server;
mod config;
mod database;
mod api;
mod mapper;

#[tokio::main]
async fn main() {
    env_logger::init();
    Server::run(Database::mongo_db_connection().await).await;
}