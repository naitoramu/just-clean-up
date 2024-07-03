use async_trait::async_trait;
use mongodb::bson::Document;
use mongodb::Collection;

use crate::entities::User;
use crate::error::http_error::HttpError;
use crate::repositories::Repository;

pub struct UserRepository {
    collection: Collection<Document>
}

#[async_trait]
impl Repository<User> for UserRepository {
    async fn get_all() -> Result<Vec<User>, HttpError> {
        todo!()
    }

    async fn get_by_id(id: u64) -> Result<User, HttpError> {
        todo!()
    }

    async fn create(entity: &User) -> Result<User, HttpError> {
        todo!()
    }

    async fn update(id: u64, entity: &User) -> Result<User, HttpError> {
        todo!()
    }

    async fn delete(id: u64) -> Result<(), HttpError> {
        todo!()
    }
}