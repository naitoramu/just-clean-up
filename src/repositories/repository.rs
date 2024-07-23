use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;

#[async_trait]
pub trait Repository<T>: Send + Sync {

    async fn find_first_matching(&self, filter: HashMap<&str, String>) -> Result<Option<T>, Box<dyn Error>>;

    async fn find_all_matching(&self, filter: HashMap<&str, String>) -> Result<Vec<T>, Box<dyn Error>>;
}
