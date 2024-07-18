use std::error::Error;

use async_trait::async_trait;

pub mod user_repository;

#[async_trait]
pub trait Repository<T>: Send + Sync {

    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

    async fn get_by_id(&self, id: String) -> Result<Option<T>, Box<dyn Error>>;

    async fn create(&self, entity: &T) -> Result<T, Box<dyn Error>>;

    async fn update(&self, id: String, entity: &T) -> Result<T, Box<dyn Error>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}