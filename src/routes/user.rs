use axum::{Json, Router};
use axum::routing::{delete, get, post, put};
use sqlx::MySqlPool;
use crate::entities::User;
use crate::repositories::Repository;
use crate::repositories::user::UserRepository;

struct UserController<'a> {
    repository: UserRepository<'a>,
}

impl<'a> UserController<'a> {

    pub fn new(db: &MySqlPool) -> Self {
        Self { repository: UserRepository::new(db) }
    }

    pub fn routes(&self, db: &MySqlPool) -> Router {
        Router::new()
            .route("/users", get(self.get_users))
        // .route("/users", post(create_user))
        // .route("/users/:id", get(get_user))
        // .route("/users/:id", put(update_user))
        // .route("/users/:id", delete(delete_user))
    }

    fn get_users(&self) -> Json<Vec<User>>{
        Json(
            self.repository.get_all()
        )
    }
}

