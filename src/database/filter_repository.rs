use std::collections::HashMap;

use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use crate::error::json_problem::JsonProblem;

#[async_trait]
pub trait FilterRepository<D>: Send + Sync {

    async fn find_by_object_id(&self, id: ObjectId) -> Result<Option<D>, JsonProblem>;

    async fn find_first_matching(&self, filter: HashMap<String, String>) -> Result<Option<D>, JsonProblem>;

    async fn find_all_matching(&self, filter: HashMap<String, String>) -> Result<Vec<D>, JsonProblem>;
}
