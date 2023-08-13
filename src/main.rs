mod entities;
mod repositories;
mod migrations;
mod error;
mod server;
mod controllers;
mod database;

use std::env;
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

pub fn development_mode() -> bool {
    env::var("DEVELOPMENT_MODE")
        .unwrap_or("false".to_string())
        .eq_ignore_ascii_case("true")
}