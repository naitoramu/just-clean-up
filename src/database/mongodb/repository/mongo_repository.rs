use std::collections::HashMap;

use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
use mongodb::Collection;

use crate::database::crud_repository::CrudRepository;
use crate::database::filter_repository::FilterRepository;
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::database::mongodb::repository::ObjectIdMapper;
use crate::domain::model::domain_model::DomainModel;
use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;

#[derive(Clone)]
pub struct MongoRepository<E: MongoEntity> {
    collection: Collection<E>,
}

impl<E> MongoRepository<E>
where
    E: MongoEntity,
{
    pub fn new(database: &mongodb::Database) -> Self {
        MongoRepository { collection: database.collection(E::get_collection_name()) }
    }
}

#[async_trait]
impl<E, D> FilterRepository<D> for MongoRepository<E>
where
    E: MongoEntity + Clone + TryFrom<D>,
    D: DomainModel + Clone + From<E>,
{
    async fn find_by_object_id(&self, id: ObjectId) -> Result<Option<D>, JsonProblem> {
        match self.collection.find_one(doc! { "_id": id }).await {
            Ok(Some(entity)) => Ok(Some(entity.into())),
            Ok(None) => Ok(None),
            Err(error) => Err(error.into())
        }
    }

    async fn find_first_matching(&self, filter: HashMap<String, String>) -> Result<Option<D>, JsonProblem> {
        match self.collection.find_one(to_document(&filter).unwrap()).await {
            Ok(Some(entity)) => Ok(Some(entity.into())),
            Ok(None) => Ok(None),
            Err(error) => Err(error.into())
        }
    }

    async fn find_all_matching(&self, filter: HashMap<String, String>) -> Result<Vec<D>, JsonProblem> {
        match self.collection.find(to_document(&filter).unwrap()).await {
            Ok(entities) => Ok(entities.try_collect::<Vec<E>>().await?.iter().map(|entity| entity.clone().into()).collect()),
            Err(error) => Err(error.into())
        }
    }
}

#[async_trait]
impl<E, D> CrudRepository<D> for MongoRepository<E>
where
    E: MongoEntity + Clone + TryFrom<D>,
    D: DomainModel + Sync + Clone + From<E>,
    <E as TryFrom<D>>::Error: Into<JsonProblem>,
{
    async fn get_all(&self) -> Result<Vec<D>, JsonProblem> {
        self.find_all_matching(HashMap::default()).await
    }

    async fn get_by_id(&self, id: String) -> Result<Option<D>, JsonProblem> {
        let object_id = ObjectIdMapper::map_to_object_id(id.as_str())?;
        self.find_by_object_id(object_id).await
    }

    async fn create(&self, model: &D) -> Result<D, JsonProblem> {
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

    async fn update(&self, id: String, model: &D) -> Result<D, JsonProblem> {
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

    async fn delete(&self, id: String) -> Result<(), JsonProblem> {
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
