use async_trait::async_trait;

// use sqlx::{query, query_as};
// use crate::database::{Database};
use crate::entities::User;
use crate::error::http_error::HttpError;
use crate::repositories::Repository;

pub struct UserRepository {}

#[async_trait]
impl Repository<User> for UserRepository {
    const SELECT_ALL_QUERY: &'static str = "";

    async fn get_all() -> Result<Vec<User>, HttpError> {
        todo!()
    }

    const SELECT_BY_ID_QUERY: &'static str = "";

    async fn get_by_id(id: u64) -> Result<User, HttpError> {
        todo!()
    }

    const INSERT_QUERY: &'static str = "";

    async fn create(entity: &User) -> Result<User, HttpError> {
        todo!()
    }

    const UPDATE_QUERY: &'static str = "";

    async fn update(id: u64, entity: &User) -> Result<User, HttpError> {
        todo!()
    }

    const DELETE_QUERY: &'static str = "";

    async fn delete(id: u64) -> Result<(), HttpError> {
        todo!()
    }
}