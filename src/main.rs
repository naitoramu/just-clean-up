mod entities;
mod migrations;
mod server;
mod routes;
mod database;

use dotenv::dotenv;
use sqlx::{MySql, Pool};
use migrations::Migrations;
use crate::database::Database;
use crate::server::Server;

#[tokio::main]
async fn main() {
    load_dot_env();
    let db: Pool<MySql> = Database::new()
        .create_db_if_not_exists().await
        .get_connection().await;
    Migrations::run_migrations(&db).await;
    Server::run().await;
}

fn load_dot_env() {
    dotenv().ok();
}