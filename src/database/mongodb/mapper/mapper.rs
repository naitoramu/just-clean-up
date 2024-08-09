use mongodb::bson::oid::ObjectId;
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::error::json_problem::JsonProblem;

pub trait Mapper<D, E> where E: MongoEntity
{

    fn map_to_entity(domain_model: D) -> Result<E, JsonProblem>;

    fn map_to_domain_model(entity: E) -> D;

    fn str_to_object_id(hex: &String) -> Result<ObjectId, JsonProblem> {
        if hex.is_empty() {
            return Ok(ObjectId::new());
        }
        ObjectId::parse_str(hex).map_err(Into::into)
    }
}
