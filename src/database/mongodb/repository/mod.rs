use std::str::FromStr;

use axum::BoxError;
use mongodb::bson::oid::ObjectId;

pub mod mongo_repository;

struct ObjectIdMapper;
impl ObjectIdMapper {

    fn map_to_object_id(id_as_str: &str) -> Result<ObjectId, BoxError> {
        match ObjectId::from_str(id_as_str) {
            Ok(object_id) => Ok(object_id),
            Err(error) => Err(error.into())
        }
    }
}