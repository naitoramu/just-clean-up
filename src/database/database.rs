use crate::database::cleaning_plan_repository::CleaningPlanRepository;
use crate::database::mongodb::mongo_database::MongoDatabase;
use crate::database::mongodb::repository::cleaning_plan_mongo_repository::CleaningPlanMongoRepository;
use crate::database::mongodb::repository::user_duty_mongo_repository::UserDutyMongoRepository;
use crate::database::mongodb::repository::user_mongo_repository::UserMongoRepository;
use crate::database::user_duty_repository::UserDutyRepository;
use crate::database::user_repository::UserRepository;
use std::sync::Arc;

pub struct Database {
    mongo_database: Option<MongoDatabase>,
}

impl Database {
    pub async fn mongo_db_connection() -> Self {
        Database {
            mongo_database: Some(
                MongoDatabase::new().await
                    .establish_connection().await
                    .expect("Cannot establish database connection")
                    .create_collections().await
                    .expect("Cannot create collection")
            )
        }
    }

    pub fn get_user_duty_repository(&self) -> Arc<dyn UserDutyRepository + Send + Sync> {
        Arc::new(UserDutyMongoRepository::new(
            self.mongo_database.as_ref()
                .expect("Database not initialized")
                .get_connection(),
        ))
    }

    pub fn get_cleaning_plan_repository(&self) -> Arc<dyn CleaningPlanRepository + Send + Sync> {
        Arc::new(CleaningPlanMongoRepository::new(
            self.mongo_database.as_ref()
                .expect("Database not initialized")
                .get_connection(),
        ))
    }

    pub fn get_user_repository(&self) -> Arc<dyn UserRepository + Send + Sync> {
        Arc::new(UserMongoRepository::new(
            self.mongo_database.as_ref()
                .expect("Database not initialized")
                .get_connection(),
        ))
    }
}