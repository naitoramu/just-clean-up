use std::collections::HashMap;
use std::marker::PhantomData;
use std::ops::Add;
use async_trait::async_trait;
use futures::TryStreamExt;
use log::debug;
use mongodb::bson::{doc, to_document, Document};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::database::crud_repository::CrudRepository;
use crate::database::read_repository::ReadRepository;
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::database::mongodb::repository::ObjectIdMapper;
use crate::domain::model::domain_model::DomainModel;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

#[derive(Clone)]
pub struct MongoRepository<D: DomainModel, E: MongoEntity> {
    collection: Collection<E>,
    _marker: PhantomData<D>
}

impl<D, E> MongoRepository<D, E>
where
    D: DomainModel + Sync + Clone + From<E>,
    E: MongoEntity + Clone + TryFrom<D>,
    <E as TryFrom<D>>::Error: Into<JsonProblem>,
{
    pub fn new(database: &mongodb::Database) -> Self {
        MongoRepository {
            collection: database.collection(E::get_collection_name()),
            _marker: PhantomData
        }
    }

    pub async fn find_by_object_id(&self, id: ObjectId) -> Result<Option<D>, JsonProblem> {
        match self.collection.find_one(doc! { "_id": id }).await {
            Ok(Some(entity)) => Ok(Some(entity.into())),
            Ok(None) => Ok(None),
            Err(error) => Err(error.into())
        }
    }

    pub async fn find_first_matching(&self, filter: Document) -> Result<Option<D>, JsonProblem> {
        match self.collection.find_one(filter).await {
            Ok(Some(entity)) => Ok(Some(entity.into())),
            Ok(None) => Ok(None),
            Err(error) => Err(error.into())
        }
    }

    pub async fn find_all_matching(&self, filter: Document) -> Result<Vec<D>, JsonProblem> {
        // let mut filter_str = "".to_string();
        // for (key, value) in filter.clone() {
        //     filter_str = filter_str + format!("({}, {})", key, value).as_str();
        // }
        // debug!("Fetching all documents matching filters '{}'", filter_str);
        match self.collection.find(filter).await {
            Ok(entities) => Ok(entities.try_collect::<Vec<E>>().await?.iter().map(|entity| entity.clone().into()).collect()),
            Err(error) => Err(error.into())
        }
    }

    pub async fn get_all(&self) -> Result<Vec<D>, JsonProblem> {
        self.find_all_matching(doc! {}).await
    }

    pub async fn get_by_id(&self, id: String) -> Result<Option<D>, JsonProblem> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.find_by_object_id(object_id).await
    }

    pub async fn create(&self, model: &D) -> Result<D, JsonProblem> {
        let entity: E = model.clone()
            .try_into()
            .map_err(Into::<JsonProblem>::into)?;

        match self.collection.insert_one(entity).await {
            Ok(insert_result) => Ok(self.find_by_object_id(
                insert_result.inserted_id.as_object_id().unwrap()
            ).await?.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    pub async fn update(&self, id: String, model: &D) -> Result<D, JsonProblem> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.ensure_resource_exists(object_id).await?;

        let entity: E = model.clone().try_into().map_err(Into::<JsonProblem>::into)?;
        let document = to_document(&entity.with_id(object_id)).unwrap();

        match self.collection.update_one(
            doc! { "_id": object_id },
            doc! { "$set": document },
        ).await {
            Ok(_) => Ok(self.find_by_object_id(object_id).await?.unwrap()),
            Err(error) => Err(error.into())
        }
    }

    pub async fn delete(&self, id: String) -> Result<(), JsonProblem> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.ensure_resource_exists(object_id).await?;

        match self.collection.delete_one(doc! { "_id": object_id }).await {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into())
        }
    }

    async fn ensure_resource_exists(&self, id: ObjectId) -> Result<(), JsonProblem> {
        if self.find_by_object_id(id).await?.is_none() {
            return Err(JsonProblems::resource_not_found(D::get_resource_name(), id.to_hex()))
        }
        Ok(())
    }
}
