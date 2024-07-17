use crate::database::Database;
use crate::server::Server;

mod entities;
mod repositories;
mod error;
mod server;
mod controllers;
mod mongo_database;
mod config;
mod database;

#[tokio::main]
async fn main() {
    Server::run(Database::mongo_db_connection().await).await;
}