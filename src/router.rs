use axum::Router;
use crate::api::controller::{auth_controller, cleaning_plan_controller, health_controller, user_controller};
use crate::database::database::Database;
use crate::domain::model::User;
use crate::middleware;

pub struct Routes;

impl Routes {
    pub fn build_routes(db: &Database, base_path: String) -> Router {
        let auth_middleware = axum::middleware::from_fn_with_state(
            db.get_repository::<User>(),
            middleware::authorization_middleware,
        );

        Router::new()
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
            ).layer(axum::middleware::from_fn(middleware::error_handling_middleware))
    }
}