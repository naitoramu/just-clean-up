use std::error::Error;
use std::str::FromStr;
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::entities::User;
use crate::error::http_error::HttpError;
use crate::repositories::Repository;

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(database: &mongodb::Database) -> Self {
        UserRepository { collection: database.collection("users") }
    }

}

#[async_trait]
impl Repository<User> for UserRepository {

    async fn get_all(&self) -> Result<Vec<User>, Box<dyn Error>> {
        match self.collection.find( doc! {} ).await {
            Ok(users) => Ok(users.try_collect().await.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    async fn get_by_id(&self, id: String) -> Result<Option<User>, Box<dyn Error>> {
        let object_id = ObjectId::from_str(id.as_str())
            .expect(format!("ID '{id}' not valid ObjectId value").as_str());

        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(user) => Ok(user),
            Err(error) => Err(error.into())
        }
    }

    async fn create(&self, entity: &User) -> Result<User, HttpError> {
        todo!()
    }

    async fn update(&self, id: String, entity: &User) -> Result<User, HttpError> {
        todo!()
    }

    async fn delete(&self, id: String) -> Result<(), HttpError> {
        todo!()
    }
}