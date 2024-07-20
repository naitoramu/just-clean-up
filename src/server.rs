use std::net::SocketAddr;

use axum::Router;
use log::{debug, info};
use crate::api::controller::{health_controller, user_controller};
use crate::database::database::Database;

pub struct Server {}

impl Server {
    pub async fn run(port: u16, base_path: String) {
        let db = Database::mongo_db_connection().await;
        debug!("Database connection established");

        let app = Router::new()
            .merge(health_controller::routes())
            .nest(base_path.as_str(), Router::new()
                .nest("/v1", Router::new()
                    .merge(user_controller::routes(&db))));

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        info!("Server started, listening on {addr}");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to start server");
    }
}