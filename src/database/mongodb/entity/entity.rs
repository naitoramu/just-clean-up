use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub trait MongoEntity: Send + Sync + Serialize + for<'a> Deserialize<'a> {
    fn with_id(self, object_id: ObjectId) -> Self;
    fn get_collection_name() -> &'static str;
}
