use crate::api::dto::cleaning_plan_dto::{CleaningPlanDto, DutyDto};
use crate::domain::model::cleaning_plan::CleaningPlan;
use crate::domain::model::duty::Duty;
use crate::api::mapper::dto_mapper::DtoMapper;

pub trait CleaningPlanMapper {}

impl DtoMapper<CleaningPlanDto, CleaningPlan> for dyn CleaningPlanMapper {

    fn map_to_domain_model(dto: CleaningPlanDto) -> CleaningPlan {
        CleaningPlan::new(
            dto.id,
            dto.title,
            dto.address,
            dto.cleaner_ids,
            dto.duties.iter().map(Self::map_duty_dto_to_entity).collect(),
            dto.start_date
        )
    }

    fn map_to_dto(entity: CleaningPlan) -> CleaningPlanDto {
        CleaningPlanDto {
            id: entity.id,
            title: entity.title,
            address: entity.address,
            cleaner_ids: entity.participant_ids,
            duties: entity.duties.iter().map(Self::map_duty_entity_to_dto).collect(),
            start_date: entity.start_date
        }
    }
}

impl dyn CleaningPlanMapper {

    fn map_duty_dto_to_entity(dto: &DutyDto) -> Duty {
        Duty::new(
            dto.id.clone(),
            dto.title.clone(),
            dto.todo_list.clone(),
            dto.img_src.clone(),
            dto.repetition.clone(),
            dto.offset.clone(),
            dto.penalty.clone(),
        )
    }

    fn map_duty_entity_to_dto(entity: &Duty) -> DutyDto {
        DutyDto {
            id: entity.id.clone(),
            title: entity.title.clone(),
            todo_list: entity.todo_list.clone(),
            img_src: entity.img_src.clone(),
            repetition: entity.repetition.clone(),
            offset: entity.offset.clone(),
            penalty: entity.penalty.clone(),
        }
    }
}
