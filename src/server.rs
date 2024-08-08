use std::net::SocketAddr;

use axum::middleware::from_fn;
use axum::Router;
use log::{debug, info};

use crate::api::controller::{auth_controller, cleaning_plan_controller, health_controller, user_controller};
use crate::database::database::Database;
use crate::domain::model::User;
use crate::middleware;
use crate::router::Routes;

pub struct Server;

impl Server {
    pub async fn run(port: u16, base_path: String) {
        let db = Database::mongo_db_connection().await;
        debug!("Database connection established");

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let listener = tokio::net::TcpListener::bind(addr).await
            .expect("Failed to bind socket");
        info!("Server started, listening on {addr}");

        let router = Routes::build_routes(&db, base_path);
        axum::serve(listener, router).await
            .expect("Failed to start server");
    }
}