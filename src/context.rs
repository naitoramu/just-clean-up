use std::sync::Arc;
use crate::database::database::Database;
use crate::domain::service::auth_service::AuthService;
use crate::domain::service::cleaning_plan_service::CleaningPlanService;
use crate::domain::service::user_service::UserService;

pub struct AppContext {
    auth_service: Arc<AuthService>,
    user_service: Arc<UserService>,
    cleaning_plan_service: Arc<CleaningPlanService>,
}

impl<'a> AppContext {

    pub fn new(database: Database) -> Self {
        Self {
            auth_service: Arc::new(AuthService::new(database.get_user_repository())),
            user_service: Arc::new(UserService::new(database.get_user_repository())),
            cleaning_plan_service: Arc::new(CleaningPlanService::new(
                database.get_user_repository(),
                database.get_cleaning_plan_repository(),
            )),
        }
    }

    pub fn get_auth_service(&self) -> Arc<AuthService> {
        Arc::clone(&self.auth_service)
    }

    pub fn get_user_service(&self) -> Arc<UserService> {
        Arc::clone(&self.user_service)
    }

    pub fn get_cleaning_plan_service(&self) -> Arc<CleaningPlanService> {
        Arc::clone(&self.cleaning_plan_service)
    }
}