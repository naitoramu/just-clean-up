use crate::api::dto::cleaning_plan_dto::{CleaningPlanDto, DutyDto};
use crate::entities::cleaning_plan::CleaningPlan;
use crate::entities::duty::Duty;
use crate::entities::penalty::Penalty;
use crate::mapper::Mapper;

pub trait CleaningPlanMapper {}

impl Mapper<CleaningPlanDto, CleaningPlan> for dyn CleaningPlanMapper {

    fn map_to_entity(dto: CleaningPlanDto) -> CleaningPlan {
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
            cleaner_ids: entity.cleaner_ids,
            duties: entity.duties.iter().map(Self::map_duty_entity_to_dto).collect(),
            start_date: entity.start_date
        }
    }
}

impl dyn CleaningPlanMapper {

    fn map_duty_dto_to_entity(dto: &DutyDto) -> Duty {
        Duty::new(
            dto.title.clone(),
            dto.description.clone(),
            dto.img_src.clone(),
            dto.repetition.clone(),
            dto.offset.clone(),
            Penalty::new(dto.penalty.clone())
        )
    }

    fn map_duty_entity_to_dto(entity: &Duty) -> DutyDto {
        DutyDto {
            title: entity.title.clone(),
            description: entity.description.clone(),
            img_src: entity.img_src.clone(),
            repetition: entity.repetition.clone(),
            offset: entity.offset.clone(),
            penalty: entity.penalty.content.clone()
        }
    }
}
