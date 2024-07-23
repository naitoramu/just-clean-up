use std::error::Error;
use std::str::FromStr;
use mongodb::bson::oid::ObjectId;

pub mod mongo_user_repository;
pub mod cleaning_plan_repository;
pub mod repository;
pub mod crud_repository;


struct ObjectIdMapper;
impl ObjectIdMapper {

    fn map_to_object_id(id_as_str: &str) -> Result<ObjectId, Box<dyn Error>> {
        match ObjectId::from_str(id_as_str) {
            Ok(object_id) => Ok(object_id),
            Err(error) => Err(error.into())
        }
    }
}