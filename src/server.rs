use std::net::SocketAddr;

use axum::middleware::from_fn;
use axum::Router;
use log::{debug, info};

use crate::api::controller::{auth_controller, cleaning_plan_controller, health_controller, user_controller};
use crate::database::database::Database;
use crate::entities::User;
use crate::middleware;

pub struct Server;

impl Server {
    pub async fn run(port: u16, base_path: String) {
        let db = Database::mongo_db_connection().await;
        debug!("Database connection established");


        let auth_middleware = axum::middleware::from_fn_with_state(
            db.get_repository::<User>(),
            middleware::authorization_middleware,
        );

        let app = Router::new()
            .merge(health_controller::routes())
            .nest(base_path.as_str(), Router::new()
                .merge(auth_controller::public_routes(&db))
                .merge(user_controller::public_routes(&db))
                .merge(auth_controller::private_routes(&db).layer(auth_middleware.clone()))
                .nest("/v1", Router::new()
                    .merge(user_controller::private_routes(&db))
                    .merge(cleaning_plan_controller::routes(&db))
                    .layer(auth_middleware),
                ),
            ).layer(from_fn(middleware::error_handling_middleware));

        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let listener = tokio::net::TcpListener::bind(addr).await
            .expect("Failed to bind socket");

        info!("Server started, listening on {addr}");
        axum::serve(listener, app).await
            .expect("Failed to start server");
    }
}