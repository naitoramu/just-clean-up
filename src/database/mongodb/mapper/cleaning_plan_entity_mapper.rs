use mongodb::bson::oid::ObjectId;
use crate::database::mongodb::entity::cleaning_plan::CleaningPlanEntity;
use crate::database::mongodb::entity::duty::DutyEntity;
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::domain::model::cleaning_plan::CleaningPlan;
use crate::domain::model::duty::Duty;
use crate::error::json_problem::JsonProblem;

pub struct CleaningPlanEntityMapper;

impl Mapper<CleaningPlan, CleaningPlanEntity> for CleaningPlanEntityMapper {

    fn map_to_entity(domain_model: CleaningPlan) -> Result<CleaningPlanEntity, JsonProblem> {
        Ok(CleaningPlanEntity {
            id: Self::str_to_object_id(&domain_model.id)?,
            title: domain_model.title,
            address: domain_model.address,
            participant_ids: domain_model.participant_ids.iter()
                .map(Self::str_to_object_id)
                .collect::<Result<Vec<ObjectId>, JsonProblem>>()?,
            duties: domain_model.duties.iter()
                .map(Self::map_duty_domain_model_to_entity)
                .collect::<Result<Vec<DutyEntity>, JsonProblem>>()?,
            start_date: domain_model.start_date,
        })
    }

    fn map_to_domain_model(entity: CleaningPlanEntity) -> CleaningPlan {
        CleaningPlan::new(
            entity.id.to_hex(),
            entity.title,
            entity.address,
            entity.participant_ids.iter().map(|user_id| user_id.to_hex()).collect(),
            entity.duties.iter().map(Self::map_duty_entity_to_domain_model).collect(),
            entity.start_date
        )
    }
}

impl CleaningPlanEntityMapper {

    fn map_duty_domain_model_to_entity(domain_model_ref: &Duty) -> Result<DutyEntity, JsonProblem> {
        let domain_model = domain_model_ref.clone();
        Ok(DutyEntity {
            id: Self::str_to_object_id(&domain_model.id)?,
            title: domain_model.title,
            todo_list: domain_model.todo_list,
            img_src: domain_model.img_src,
            repetition: domain_model.repetition,
            offset: domain_model.offset,
            penalty: domain_model.penalty,
        })
    }

    fn map_duty_entity_to_domain_model(entity_ref: &DutyEntity) -> Duty {
        let entity = entity_ref.clone();
        Duty::new(
            entity.id.to_hex(),
            entity.title,
            entity.todo_list,
            entity.img_src,
            entity.repetition,
            entity.offset,
            entity.penalty,
        )
    }
}