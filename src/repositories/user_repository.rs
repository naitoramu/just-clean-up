use std::error::Error;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::entities::User;
use crate::error::json_problems::JsonProblems;
use crate::repositories::{ObjectIdMapper, Repository};

#[derive(Clone)]
pub struct UserRepository {
    collection: Collection<User>,
}

impl UserRepository {
    pub fn new(database: &mongodb::Database) -> Self {
        UserRepository { collection: database.collection("users") }
    }

    async fn get_by_object_id(&self, object_id: ObjectId) -> Result<User, Box<dyn Error>> {
        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(user) => match user {
                None => Err(Box::new(JsonProblems::resource_not_found("User", object_id.to_hex()))),
                Some(fetched_user) => Ok(fetched_user)
            },
            Err(error) => Err(error.into())
        }
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
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await
    }

    async fn create(&self, entity: &User) -> Result<User, Box<dyn Error>> {
        match self.collection.insert_one(entity).await {
            Ok(insert_result) => Ok(
                self.get_by_object_id(insert_result.inserted_id.as_object_id().unwrap()).await?
            ),
            Err(error) => Err(error.into())
        }
    }

    async fn update(&self, id: String, entity: &User) -> Result<User, Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await?;

        let user_document = to_document(entity).unwrap();

        match self.collection.update_one(
            doc! { "_id": object_id },
            doc! { "$set": user_document }
        ).await {
            Ok(_) => self.get_by_object_id(object_id).await,
            Err(error) => Err(error.into())
        }
    }

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await?;

        match self.collection.delete_one( doc! { "_id": object_id }).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into())
        }
    }
}