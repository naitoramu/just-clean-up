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
    // Database::new()
    //     .create_db_if_not_exists().await
    //     .establish_connection().await;
    Server::run().await;
}