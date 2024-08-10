use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::database::mongodb::entity::entity::MongoEntity;

#[derive(Serialize, Deserialize, Clone)]
pub struct DutyEntity {

    #[serde(rename = "_id")]
    pub id: ObjectId,

    pub title: String,

    pub todo_list: Vec<String>,

    pub img_src: Option<String>,

    pub repetition: String,

    pub offset: String,

    pub penalty: String,
}

impl MongoEntity for DutyEntity {

    fn with_id(mut self, object_id: ObjectId) -> Self {
        self.id = object_id;
        self
    }

    fn get_collection_name() -> &'static str {
        "duties"
    }
}