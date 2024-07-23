use std::sync::Arc;

use crate::database::mongo_database::MongoDatabase;
use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::User;
use crate::repositories::cleaning_plan_repository::CleaningPlanRepository;
use crate::repositories::crud_repository::CrudRepository;
use crate::repositories::mongo_user_repository::MongoUserRepository;

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

    pub fn get_user_repository(&self) -> Arc<dyn CrudRepository<User>> {
        Arc::new(MongoUserRepository::new(
            self.mongo_database.as_ref().expect("Database not initialized").get_connection(),
        ))
    }

    pub fn get_cleaning_plan_repository(&self) -> Arc<dyn CrudRepository<CleaningPlan>> {
        Arc::new(CleaningPlanRepository::new(
            self.mongo_database.as_ref().expect("Database not initialized").get_connection(),
        ))
    }
}