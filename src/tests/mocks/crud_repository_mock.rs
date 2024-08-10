use crate::database::crud_repository::CrudRepository;
use crate::database::read_repository::ReadRepository;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use pseudo::Mock;
use std::collections::HashMap;

pub struct CrudRepositoryMock<D>
where
    D: Clone + 'static,
{
    pub find_by_object_id_fn: Mock<ObjectId, Result<Option<D>, JsonProblem>>,
    pub find_first_matching_fn: Mock<HashMap<String, String>, Result<Option<D>, JsonProblem>>,
    pub find_all_matching_fn: Mock<HashMap<String, String>, Result<Vec<D>, JsonProblem>>,

    pub get_all_fn: Mock<(), Result<Vec<D>, JsonProblem>>,
    pub get_by_id: Mock<String, Result<Option<D>, JsonProblem>>,
    pub create_fn: Mock<D, Result<D, JsonProblem>>,
    pub update_fn: Mock<(String, D), Result<D, JsonProblem>>,
    pub delete_fn: Mock<String, Result<(), JsonProblem>>,
}

impl<D> CrudRepositoryMock<D>
where
    D: Clone,
{
    pub fn new() -> Self {
        Self {
            get_all_fn: Mock::new(Err(JsonProblems::not_implemented())),
            get_by_id: Mock::new(Err(JsonProblems::not_implemented())),
            create_fn: Mock::new(Err(JsonProblems::not_implemented())),
            update_fn: Mock::new(Err(JsonProblems::not_implemented())),
            delete_fn: Mock::new(Err(JsonProblems::not_implemented())),
            find_by_object_id_fn: Mock::new(Err(JsonProblems::not_implemented())),
            find_first_matching_fn: Mock::new(Err(JsonProblems::not_implemented())),
            find_all_matching_fn: Mock::new(Err(JsonProblems::not_implemented())),
        }
    }
}

#[async_trait]
impl<D> ReadRepository<D> for CrudRepositoryMock<D>
where
    D: Clone + Sync + Send,
{
    async fn find_by_object_id(&self, id: ObjectId) -> Result<Option<D>, JsonProblem> {
        self.find_by_object_id_fn.call(id)
    }

    async fn find_first_matching(&self, filter: HashMap<String, String>) -> Result<Option<D>, JsonProblem> {
        self.find_first_matching_fn.call(filter)
    }

    async fn find_all_matching(&self, filter: HashMap<String, String>) -> Result<Vec<D>, JsonProblem> {
        self.find_all_matching_fn.call(filter)
    }
}

#[async_trait]
impl<D> CrudRepository<D> for CrudRepositoryMock<D>
where
    D: Clone + Sync + Send,
{
    async fn get_all(&self) -> Result<Vec<D>, JsonProblem> {
        self.get_all_fn.call(())
    }
    async fn get_by_id(&self, id: String) -> Result<Option<D>, JsonProblem> {
        self.get_by_id.call(id)
    }
    async fn create(&self, model: &D) -> Result<D, JsonProblem> {
        self.create_fn.call(model.clone())
    }
    async fn update(&self, id: String, model: &D) -> Result<D, JsonProblem> {
        self.update_fn.call((id, model.clone()))
    }
    async fn delete(&self, id: String) -> Result<(), JsonProblem> {
        self.delete_fn.call(id)
    }
    async fn ensure_resource_exists(&self, id: ObjectId) -> Result<(), JsonProblem> {
        todo!()
    }
}
