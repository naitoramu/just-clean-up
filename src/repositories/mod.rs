use std::error::Error;
use async_trait::async_trait;

use crate::error::http_error::HttpError;

pub mod user_repository;

#[async_trait]
pub trait Repository<T>: Send + Sync {

    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

    async fn get_by_id(&self, id: String) -> Result<Option<T>, Box<dyn Error>>;

    async fn create(&self, entity: &T) -> Result<T, HttpError>;

    async fn update(&self, id: String, entity: &T) -> Result<T, HttpError>;

    async fn delete(&self, id: String) -> Result<(), HttpError>;
}