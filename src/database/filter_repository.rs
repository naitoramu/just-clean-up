use std::collections::HashMap;

use async_trait::async_trait;
use axum::BoxError;
use mongodb::bson::oid::ObjectId;

#[async_trait]
pub trait FilterRepository<D>: Send + Sync {
    async fn find_by_object_id(&self, id: ObjectId) -> Result<Option<D>, BoxError>;

    async fn find_first_matching(&self, filter: HashMap<&str, String>) -> Result<Option<D>, BoxError>;

    async fn find_all_matching(&self, filter: HashMap<&str, String>) -> Result<Vec<D>, BoxError>;
}
