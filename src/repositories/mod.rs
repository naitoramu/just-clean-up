use async_trait::async_trait;
use crate::error::http_error::HttpError;

pub mod user_repository;

#[async_trait]
pub trait Repository<T> {

    async fn get_all() -> Result<Vec<T>, HttpError>;

    async fn get_by_id(id: u64) -> Result<T, HttpError>;

    async fn create(entity: &T) -> Result<T, HttpError>;

    async fn update(id: u64, entity: &T) -> Result<T, HttpError>;

    async fn delete(id: u64) -> Result<(), HttpError>;
}