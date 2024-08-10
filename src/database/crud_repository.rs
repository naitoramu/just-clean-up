use crate::database::read_repository::ReadRepository;
use crate::error::json_problem::JsonProblem;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

#[async_trait]
pub trait CrudRepository<T>: ReadRepository<T> {

    async fn get_all(&self) -> Result<Vec<T>, JsonProblem>;

    async fn get_by_id(&self, id: String) -> Result<Option<T>, JsonProblem>;

    async fn create(&self, model: &T) -> Result<T, JsonProblem>;

    async fn update(&self, id: String, model: &T) -> Result<T, JsonProblem>;

    async fn delete(&self, id: String) -> Result<(), JsonProblem>;

    async fn ensure_resource_exists(&self, id: ObjectId) -> Result<(), JsonProblem>;
}