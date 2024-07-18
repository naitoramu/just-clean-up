use std::error::Error;
use std::str::FromStr;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

pub mod user_repository;

#[async_trait]
pub trait Repository<T>: Send + Sync {

    async fn get_all(&self) -> Result<Vec<T>, Box<dyn Error>>;

    async fn get_by_id(&self, id: String) -> Result<T, Box<dyn Error>>;

    async fn create(&self, entity: &T) -> Result<T, Box<dyn Error>>;

    async fn update(&self, id: String, entity: &T) -> Result<T, Box<dyn Error>>;

    async fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}


struct ObjectIdMapper;
impl ObjectIdMapper {

    fn map_to_object_id(id_as_str: &str) -> Result<ObjectId, Box<dyn Error>> {
        match ObjectId::from_str(id_as_str) {
            Ok(object_id) => Ok(object_id),
            Err(error) => Err(error.into())
        }
    }
}