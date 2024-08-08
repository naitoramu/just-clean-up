use async_trait::async_trait;
use axum::BoxError;
use mongodb::bson::oid::ObjectId;

use crate::database::filter_repository::FilterRepository;
use crate::error::json_problem::JsonProblem;

#[async_trait]
pub trait CrudRepository<T>: FilterRepository<T> {

    async fn get_all(&self) -> Result<Vec<T>, BoxError>;

    async fn get_by_id(&self, id: String) -> Result<Option<T>, BoxError>;

    async fn create(&self, model: &T) -> Result<T, BoxError>;

    async fn update(&self, id: String, model: &T) -> Result<T, BoxError>;

    async fn delete(&self, id: String) -> Result<(), BoxError>;
    async fn ensure_resource_exists(&self, id: ObjectId) -> Result<(), JsonProblem>;
}