mod entities;
mod migrations;
mod server;

use dotenv::dotenv;
use migrations::Migrations;

#[tokio::main]
async fn main() {
    load_dot_env();
    Migrations::migrate().await;
}

fn load_dot_env() {
    dotenv().ok();
}