use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use crate::error::json_problem::JsonProblem;

pub mod mongo_repository;

struct ObjectIdMapper;
impl ObjectIdMapper {

    fn map_to_object_id(id_as_str: &str) -> Result<ObjectId, JsonProblem> {
        match ObjectId::from_str(id_as_str) {
            Ok(object_id) => Ok(object_id),
            Err(error) => Err(error.into())
        }
    }
}