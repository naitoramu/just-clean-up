use async_trait::async_trait;
use std::error::Error;
use crate::repositories::filter_repository::FilterRepository;

#[async_trait]
pub trait CrudRepository<T>: FilterRepository<T> {

    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

    async fn get_by_id(&self, id: String) -> Result<Option<T>, Box<dyn Error>>;

    async fn create(&self, entity: &T) -> Result<T, Box<dyn Error>>;

    async fn update(&self, id: String, entity: &T) -> Result<T, Box<dyn Error>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}