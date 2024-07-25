use std::collections::HashMap;

use async_trait::async_trait;
use axum::BoxError;

#[async_trait]
pub trait FilterRepository<T>: Send + Sync {

    async fn find_first_matching(&self, filter: HashMap<&str, String>) -> Result<Option<T>, BoxError>;

    async fn find_all_matching(&self, filter: HashMap<&str, String>) -> Result<Vec<T>, BoxError>;
}
