use crate::database::Database;
use crate::server::Server;

mod entities;
mod repositories;
mod error;
mod server;
mod mongo_database;
mod config;
mod database;
mod api;
mod mapper;

#[tokio::main]
async fn main() {
    Server::run(Database::mongo_db_connection().await).await;
}