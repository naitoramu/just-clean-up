use crate::database::cleaning_plan_repository::CleaningPlanRepository;
use crate::database::mongodb::entity::cleaning_plan_entity::CleaningPlanEntity;
use crate::database::mongodb::repository::mongo_repository::MongoRepository;
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::error::json_problem::JsonProblem;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;

pub struct CleaningPlanMongoRepository {
    repository: MongoRepository<CleaningPlan, CleaningPlanEntity>,
}

impl CleaningPlanMongoRepository {
    pub fn new(database: &Database) -> Self {
        CleaningPlanMongoRepository {
            repository: MongoRepository::<CleaningPlan, CleaningPlanEntity>::new(database)
        }
    }
}

#[async_trait]
impl CleaningPlanRepository for CleaningPlanMongoRepository {
    async fn get_all_plans(&self) -> Result<Vec<CleaningPlan>, JsonProblem> {
        self.repository.get_all().await
    }

    async fn get_plan_by_id(&self, id: String) -> Result<Option<CleaningPlan>, JsonProblem> {
        self.repository.get_by_id(id).await
    }

    async fn get_plans_with_status(&self, status: CleaningPlanStatus) -> Result<Vec<CleaningPlan>, JsonProblem> {
        self.repository.find_all_matching(doc! {
            "status": status.to_string()
        }).await
    }

    async fn create_plan(&self, user: &CleaningPlan) -> Result<CleaningPlan, JsonProblem> {
        self.repository.create(user).await
    }

    async fn update_plan(&self, id: String, user: &CleaningPlan) -> Result<CleaningPlan, JsonProblem> {
        self.repository.update(id, user).await
    }

    async fn delete_plan(&self, id: String) -> Result<(), JsonProblem> {
        self.repository.delete(id).await
    }
}