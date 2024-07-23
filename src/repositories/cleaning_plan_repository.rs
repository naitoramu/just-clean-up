use std::error::Error;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::User;
use crate::error::json_problems::JsonProblems;
use crate::repositories::{ObjectIdMapper, Repository};

#[derive(Clone)]
pub struct CleaningPlanRepository {
    collection: Collection<CleaningPlan>,
}

impl CleaningPlanRepository {

    pub fn new(database: &mongodb::Database) -> Self {
        CleaningPlanRepository { collection: database.collection("cleaning_plans") }
    }

    async fn get_by_object_id(&self, object_id: ObjectId) -> Result<CleaningPlan, Box<dyn Error>> {
        match self.collection.find_one(doc! { "_id": object_id }).await {
            Ok(plan) => match plan {
                None => Err(Box::new(JsonProblems::resource_not_found("Cleaning Plan", object_id.to_hex()))),
                Some(fetched_plan) => Ok(fetched_plan)
            },
            Err(error) => Err(error.into())
        }
    }
}

#[async_trait]
impl Repository<CleaningPlan> for CleaningPlanRepository {

    async fn get_all(&self) -> Result<Vec<CleaningPlan>, Box<dyn Error>> {
        match self.collection.find(doc! {}).await {
            Ok(plan) => Ok(plan.try_collect().await.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    async fn get_by_id(&self, id: String) -> Result<CleaningPlan, Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await
    }

    async fn create(&self, entity: &CleaningPlan) -> Result<CleaningPlan, Box<dyn Error>> {
        match self.collection.insert_one(entity).await {
            Ok(insert_result) => Ok(
                self.get_by_object_id(insert_result.inserted_id.as_object_id().unwrap()).await?
            ),
            Err(error) => Err(error.into())
        }
    }

    async fn update(&self, id: String, entity: &CleaningPlan) -> Result<CleaningPlan, Box<dyn Error>> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.get_by_object_id(object_id).await?;

        let plan_document = to_document(entity).unwrap();

        match self.collection.update_one(
            doc! { "_id": object_id },
            doc! { "$set": plan_document }
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