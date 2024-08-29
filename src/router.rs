use axum::Router;

use crate::api::controller::{auth_controller, cleaning_plan_controller, health_controller, internal_controller, user_controller};
use crate::context::AppContext;
use crate::middleware;

pub struct Routes;

impl Routes {
    pub fn build_routes(app_context: AppContext, base_path: String) -> Router {
        let auth_middleware = axum::middleware::from_fn_with_state(
            app_context.get_auth_service(),
            middleware::authorization_middleware,
        );

        Router::new()
            .merge(health_controller::routes())
            .nest(base_path.as_str(), Router::new()
                .merge(auth_controller::public_routes(&app_context))
                .merge(user_controller::public_routes(&app_context))
                .merge(auth_controller::private_routes(&app_context).layer(auth_middleware.clone()))
                .nest("/internal", Router::new()
                    .merge(internal_controller::routes(&app_context))
                    .layer(auth_middleware.clone()))
                .nest("/v1", Router::new()
                    .merge(user_controller::private_routes(&app_context))
                    .merge(cleaning_plan_controller::routes(&app_context))
                    .layer(auth_middleware)),
            ).layer(axum::middleware::from_fn(middleware::error_handling_middleware))
    }
}