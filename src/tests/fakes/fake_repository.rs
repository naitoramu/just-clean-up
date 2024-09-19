use crate::domain::model::domain_model::DomainModel;
use crate::error::json_problem::JsonProblem;
use std::slice::Iter;

#[derive(Clone)]
pub struct FakeRepository<T> {
    entities: Vec<T>
}

impl<T: Clone + DomainModel> FakeRepository<T> {

    pub fn new() -> Self {
        FakeRepository { entities: Vec::new() }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        self.entities.iter()
    }

    pub fn entities(&self) -> Vec<T> {
        self.entities.clone()
    }

    pub fn insert(&mut self, entity: T) -> Result<T, JsonProblem> {
        self.entities.push(entity.clone());
        Ok(entity)
    }

    pub fn update(&mut self, id: String, entity: T) -> Result<T, JsonProblem> {
        self.delete_by_id(id)?;
        self.entities.push(entity.clone());
        Ok(entity)
    }

    pub fn delete_by_id(&mut self, id: String) -> Result<(), JsonProblem> {
        let index = self.get_entity_index(id);
        self.entities.remove(index);
        Ok(())
    }

    fn get_entity_index(&self, entity_id: String) -> usize {
        self.entities.iter().position(|entity| *entity.id() == entity_id).unwrap()
    }

    pub fn set_entities(&mut self, entities: Vec<T>) {
        self.entities = entities;
    }
}