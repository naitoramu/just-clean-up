use axum::BoxError;
use mongodb::bson::oid::ObjectId;
use crate::database::mongodb::entity::entity::MongoEntity;

pub trait Mapper<D, E> where E: MongoEntity
{

    fn map_to_entity(domain_model: D) -> Result<E, BoxError>;

    fn map_to_domain_model(entity: E) -> D;

    fn str_to_object_id(hex: String) -> Result<ObjectId, BoxError> {
        if hex.is_empty() {
            return Ok(ObjectId::new());
        }
        ObjectId::parse_str(hex).map_err(Into::into)
    }
}
