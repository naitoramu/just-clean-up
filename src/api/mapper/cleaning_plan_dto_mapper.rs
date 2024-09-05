use chrono::DateTime;
use crate::api::dto::cleaning_plan_dto::{CleaningPlanDto, DutyDto, RoutineDto};
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::domain::model::duty::{Duties, Duty};
use crate::api::mapper::dto_mapper::DtoMapper;
use crate::domain::model::routines::{Routine, Routines};
use crate::domain::model::time_duration::TimeDuration;

pub trait CleaningPlanMapper {}

impl DtoMapper<CleaningPlanDto, CleaningPlan> for dyn CleaningPlanMapper {
    fn map_to_domain_model(dto: CleaningPlanDto) -> CleaningPlan {
        CleaningPlan::new(
            dto.id,
            dto.title,
            dto.address,
            dto.cleaner_ids,
            Self::map_dto_routines(dto.routines),
            DateTime::parse_from_rfc3339(dto.start_date.as_str()).expect("Unable to parse str to DateTime").to_utc(),
            CleaningPlanStatus::PendingDutyAssignment,
        )
    }

    fn map_to_dto(entity: CleaningPlan) -> CleaningPlanDto {
        CleaningPlanDto {
            id: entity.id,
            title: entity.title,
            address: entity.address,
            cleaner_ids: entity.participant_ids,
            routines: Self::map_domain_routines(entity.routines),
            start_date: entity.start_date.to_rfc3339(),
        }
    }
}

impl dyn CleaningPlanMapper {
    fn map_dto_routines(routines: Vec<RoutineDto>) -> Routines {
        let mut mapped_routines = Vec::new();

        for routine in routines {
            mapped_routines.push(Routine::new(
                TimeDuration::from_str(routine.repetition).expect("Unable to parse str to TimeDuration"),
                TimeDuration::from_str(routine.offset).expect("Unable to parse str to TimeDuration"),
                Self::map_dto_duties(routine.duties),
            ))
        }

        Routines::new(mapped_routines)
    }

    fn map_dto_duties(duties: Vec<DutyDto>) -> Duties {
        let mut mapped_duties = Vec::new();

        for duty in duties {
            mapped_duties.push(Duty::new(
                duty.id,
                duty.title,
                duty.todo_list,
                duty.img_src,
                duty.penalty,
            ))
        }

        Duties::new(mapped_duties)
    }

    fn map_domain_routines(routines: Routines) -> Vec<RoutineDto> {
        let mut mapped_routines = Vec::new();

        for routine in routines.vec() {
            mapped_routines.push(RoutineDto {
                repetition: routine.repetition.to_string(),
                offset: routine.offset.to_string(),
                duties: Self::map_domain_duties(routine.duties),
            })
        }

        mapped_routines
    }

    fn map_domain_duties(duties: Duties) -> Vec<DutyDto> {
        let mut mapped_duties = Vec::new();

        for duty in duties.vec() {
            mapped_duties.push(DutyDto {
                id: duty.id,
                title: duty.title,
                todo_list: duty.todo_list,
                img_src: duty.img_src,
                penalty: duty.penalty,
            })
        }

        mapped_duties
    }
}
