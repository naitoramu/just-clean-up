use crate::database::Database;
use crate::server::Server;

mod entities;
mod repositories;
mod error;
mod server;
mod controllers;
mod database;
mod config;

#[tokio::main]
async fn main() {
    Database::new().establish_connection().await.expect("Cannot establish database connection");
    Server::run().await;
}