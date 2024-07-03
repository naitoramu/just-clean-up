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
    Database::new().await
        .establish_connection().await
        .expect("Cannot establish database connection")
        .create_collections().await
        .expect("Cannot create collection");

    Server::run().await;
}