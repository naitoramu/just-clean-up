use std::sync::{Mutex, MutexGuard};
use crate::database::user_duty_repository::UserDutyRepository;
use crate::domain::model::user_duty::UserDuty;
use crate::error::json_problem::JsonProblem;
use crate::tests::fakes::fake_repository::FakeRepository;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;

pub struct UserDutyFakeRepository {
    fake_repo: Mutex<FakeRepository<UserDuty>>
}

impl UserDutyFakeRepository {

    pub fn new() -> Self {
        UserDutyFakeRepository { fake_repo: Mutex::new(FakeRepository::new()) }
    }

    pub fn set_user_duties(&self, duties: Vec<UserDuty>) {
        self.fake_repo().set_entities(duties)
    }

    fn fake_repo(&self) -> MutexGuard<'_, FakeRepository<UserDuty>> {
        self.fake_repo.lock().unwrap()
    }
}

#[async_trait]
impl UserDutyRepository for UserDutyFakeRepository {

    async fn get_all_duties(&self) -> Result<Vec<UserDuty>, JsonProblem> {
        Ok(self.fake_repo().entities())
    }

    async fn get_all_user_duties(&self, user_id: String) -> Result<Vec<UserDuty>, JsonProblem> {
        Ok(self.fake_repo().iter()
            .filter(|d| d.user_id == user_id)
            .map(|d| d.clone())
            .collect())
    }

    async fn get_user_duties_by_duty_template(&self, user_id: String, template_id: String) -> Result<Vec<UserDuty>, JsonProblem> {
        Ok(self.fake_repo().iter()
            .filter(|d| d.user_id == user_id && d.template_id == template_id)
            .map(|d| d.clone())
            .collect())
    }

    async fn get_user_duty_by_id(&self, id: String) -> Result<Option<UserDuty>, JsonProblem> {
        match self.fake_repo().iter().find(|d| d.id == id) {
            Some(duty) => Ok(Some(duty.clone())),
            None => Ok(None)
        }
    }

    async fn create_user_duty(&self, user_duty: &UserDuty) -> Result<UserDuty, JsonProblem> {
        let mut new_duty = user_duty.clone();
        let object_id = ObjectId::new().to_hex();
        new_duty.id = object_id.clone();

        self.fake_repo().insert(new_duty)
    }

    async fn update_user_duty(&self, id: String, user_duty: &UserDuty) -> Result<UserDuty, JsonProblem> {
        let mut new_user_duty = user_duty.clone();
        new_user_duty.id = id.clone();

        self.fake_repo().update(id, new_user_duty)
    }

    async fn delete_user_duty(&self, id: String) -> Result<(), JsonProblem> {
        self.fake_repo().delete_by_id(id)
    }
}