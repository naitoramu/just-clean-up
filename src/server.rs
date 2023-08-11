use axum::Router;
use std::{net::SocketAddr};
use crate::controllers::user_controller;

pub struct Server {}

impl Server {
    pub async fn run() {
        let app = Router::new()
            .nest("/v1", Router::new()
                .merge(user_controller::routes()));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
        println!("Server started, listening on {addr}");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Failed to start server");
    }
}