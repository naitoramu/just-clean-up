use async_trait::async_trait;
use crate::error::http_error::HttpError;

pub mod user_repository;

#[async_trait]
pub trait CrudRepository<T> {
    const SELECT_ALL_QUERY: &'static str;
    async fn get_all() -> Result<Vec<T>, HttpError>;

    const SELECT_BY_ID_QUERY: &'static str;
    async fn get_by_id(id: u64) -> Result<T, HttpError>;

    const INSERT_QUERY: &'static str;
    async fn create(entity: &T) -> Result<T, HttpError>;

    const UPDATE_QUERY: &'static str;
    async fn update(id: u64, entity: &T) -> Result<T, HttpError>;

    const DELETE_QUERY: &'static str;
    async fn delete(id: u64) -> Result<(), HttpError>;
}