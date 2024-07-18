use std::error::Error;
use std::str::FromStr;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::entities::User;
use crate::error::http_error::HttpError;
use crate::error::http_error_kind::HttpErrorKind;
use crate::repositories::{ObjectIdMapper, Repository};

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
        match self.collection.find(doc! {}).await {
            Ok(users) => Ok(users.try_collect().await.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    async fn get_by_id(&self, id: String) -> Result<User, Box<dyn Error>> {
        let object_id = ObjectId::from_str(id.as_str())
            .expect(format!("ID '{id}' not valid ObjectId value").as_str());

        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(user) => match user {
                None => Err(Box::new(HttpError::from_type(HttpErrorKind::ResourceNotFound(id)))),
                Some(fetched_user) => Ok(fetched_user)
            },
            Err(error) => Err(error.into())
        }
    }

    async fn create(&self, entity: &User) -> Result<User, Box<dyn Error>> {
        match self.collection.insert_one(entity).await {
            Ok(insert_result) => Ok(
                self.get_by_id(insert_result.inserted_id.as_object_id().unwrap().to_hex()).await?
            ),
            Err(error) => Err(error.into())
        }
    }

    async fn update(&self, id: String, entity: &User) -> Result<User, Box<dyn Error>> {
        self.get_by_id(id.clone()).await?;

        let object_id = ObjectIdMapper::map_to_object_id(id.as_str());
        let user_document = to_document(entity).unwrap();

        match self.collection.update_one( doc! { "_id": object_id }, doc! { "$set": user_document }).await {
            Ok(_) => self.get_by_id(id).await,
            Err(error) => Err(error.into())
        }
    }

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        self.get_by_id(id.clone()).await?;
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str());

        match self.collection.delete_one( doc! { "_id": object_id }).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into())
        }
    }
}