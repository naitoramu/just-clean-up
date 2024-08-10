use std::net::SocketAddr;

use log::{debug, info};
use crate::context::AppContext;
use crate::database::database::Database;
use crate::router::Routes;

pub struct Server;

impl Server {
    pub async fn run(port: u16, base_path: String) {
        let db = Database::mongo_db_connection().await;
        debug!("Database connection established");

        let app_context = AppContext::new(db);
        debug!("Application context created");

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let listener = tokio::net::TcpListener::bind(addr).await
            .expect("Failed to bind socket");
        info!("Server started, listening on {addr}");

        let router = Routes::build_routes(app_context, base_path);
        axum::serve(listener, router).await
            .expect("Failed to start server");
    }
}