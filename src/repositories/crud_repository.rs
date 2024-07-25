use async_trait::async_trait;
use axum::BoxError;

use crate::repositories::filter_repository::FilterRepository;

#[async_trait]
pub trait CrudRepository<T>: FilterRepository<T> {

    async fn get_all(&self) -> Result<Vec<T>, BoxError>;

    async fn get_by_id(&self, id: String) -> Result<Option<T>, BoxError>;

    async fn create(&self, entity: &T) -> Result<T, BoxError>;

    async fn update(&self, id: String, entity: &T) -> Result<T, BoxError>;

    async fn delete(&self, id: String) -> Result<(), BoxError>;
}