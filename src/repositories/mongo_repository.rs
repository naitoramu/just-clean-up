use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::entities::Entity;
use crate::repositories::crud_repository::CrudRepository;
use crate::repositories::filter_repository::FilterRepository;
use crate::repositories::ObjectIdMapper;

#[derive(Clone)]
pub struct MongoRepository<T: Entity> {
    collection: Collection<T>,
}

impl<T> MongoRepository<T>
where
    T: Entity,
{
    pub fn new(database: &mongodb::Database) -> Self {
        MongoRepository { collection: database.collection(T::get_collection_name()) }
    }

    async fn get_by_object_id(&self, object_id: ObjectId) -> Result<Option<T>, Box<dyn Error>> {
        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(entity) => Ok(entity),
            Err(error) => Err(error.into())
        }
    }
}

#[async_trait]
impl<T> FilterRepository<T> for MongoRepository<T>
where
    T: Entity,
{
    async fn find_first_matching(&self, filter: HashMap<&str, String>) -> Result<Option<T>, Box<dyn Error>> {
        match self.collection.find_one(to_document(&filter).unwrap()).await {
            Ok(entity) => Ok(entity),
            Err(error) => Err(error.into())
        }
    }

    async fn find_all_matching(&self, filter: HashMap<&str, String>) -> Result<Vec<T>, Box<dyn Error>> {
        match self.collection.find(to_document(&filter).unwrap()).await {
            Ok(entities) => Ok(entities.try_collect().await.unwrap()),
            Err(error) => Err(error.into())
        }
    }
}

#[async_trait]
impl<T> CrudRepository<T> for MongoRepository<T>
where
    T: Entity,
{
    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        match self.collection.find(doc! {}).await {
            Ok(entities) => Ok(entities.try_collect().await.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    async fn get_by_id(&self, id: String) -> Result<Option<T>, Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await
    }

    async fn create(&self, entity: &T) -> Result<T, Box<dyn Error>> {
        match self.collection.insert_one(entity).await {
            Ok(insert_result) => Ok(self.get_by_object_id(
                insert_result.inserted_id.as_object_id().unwrap()
            ).await?.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    async fn update(&self, id: String, entity: &T) -> Result<T, Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await?;

        let document = to_document(entity).unwrap();

        match self.collection.update_one(
            doc! { "_id": object_id },
            doc! { "$set": document },
        ).await {
            Ok(_) => Ok(self.get_by_object_id(object_id).await?.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await?;

        match self.collection.delete_one(doc! { "_id": object_id }).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into())
        }
    }
}