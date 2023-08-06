mod entities;
mod repositories;
mod migrations;
mod error;
mod server;
mod routes;
mod database;

use dotenv::dotenv;
use migrations::Migrations;
use crate::database::{CONNECTION, Database};
use crate::server::Server;

#[tokio::main]
async fn main() {
    load_dot_env();
    Database::new()
        .create_db_if_not_exists().await
        .establish_connection().await;
    Migrations::run_migrations(CONNECTION.get().unwrap()).await;
    Server::run().await;
}

fn load_dot_env() {
    dotenv().ok();
}