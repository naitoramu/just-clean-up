use chrono::DateTime;
use mongodb::bson::oid::ObjectId;
use crate::database::mongodb::entity::cleaning_plan_entity::CleaningPlanEntity;
use crate::database::mongodb::entity::duty_entity::DutyEntity;
use crate::database::mongodb::entity::routine_entity::RoutineEntity;
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::domain::model::duty::{Duties, Duty};
use crate::domain::model::routines::{Routine, Routines};
use crate::domain::model::time_duration::TimeDuration;
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
            routines: Self::map_domain_routines(domain_model.routines)?,
            start_date: domain_model.start_date.timestamp(),
            status: domain_model.status.to_string(),
        })
    }

    fn map_to_domain_model(entity: CleaningPlanEntity) -> CleaningPlan {
        CleaningPlan::new(
            entity.id.to_hex(),
            entity.title,
            entity.address,
            entity.participant_ids.iter().map(|user_id| user_id.to_hex()).collect(),
            Self::map_entity_routines(entity.routines),
            DateTime::from_timestamp(entity.start_date, 0)
                .expect(format!("Failed to parse timestamp '{}'", entity.start_date).as_str()),
            CleaningPlanStatus::from_string(entity.status),
        )
    }
}

impl CleaningPlanEntityMapper {
    fn map_domain_routines(routines: Routines) -> Result<Vec<RoutineEntity>, JsonProblem> {
        let mut mapped_routines = Vec::new();

        for routine in routines.vec() {
            mapped_routines.push(RoutineEntity {
                repetition: routine.repetition.to_string(),
                offset: routine.offset.to_string(),
                duties: Self::map_domain_duties(routine.duties)?,
            });
        }

        Ok(mapped_routines)
    }

    fn map_domain_duties(duties: Duties) -> Result<Vec<DutyEntity>, JsonProblem> {
        let mut mapped_duties = Vec::new();

        for duty in duties.vec() {
            mapped_duties.push(DutyEntity {
                id: Self::str_to_object_id(&duty.id)?,
                title: duty.title,
                todo_list: duty.todo_list,
                img_src: duty.img_src,
                penalty: duty.penalty,
                creation_timestamp: duty.creation_time.timestamp()
            });
        }

        Ok(mapped_duties)
    }

    fn map_entity_routines(routines: Vec<RoutineEntity>) -> Routines {
        let mut mapped_routines = Vec::new();

        for routine in routines {
            mapped_routines.push(Routine::new(
                // TODO change expects to failable mappers
                TimeDuration::from_str(routine.repetition).expect("Unable to parse str to TimeDuration"),
                TimeDuration::from_str(routine.offset).expect("Unable to parse str to TimeDuration"),
                Self::map_entity_duties(routine.duties),
            ))
        }

        Routines::new(mapped_routines)
    }

    fn map_entity_duties(duties: Vec<DutyEntity>) -> Duties {
        let mut mapped_duties = Vec::new();

        for duty in duties {
            mapped_duties.push(Duty::new(
                duty.id.to_hex(),
                duty.title,
                duty.todo_list,
                duty.img_src,
                duty.penalty,
                DateTime::from_timestamp(duty.creation_timestamp, 0)
                    .expect(format!("Failed to parse timestamp '{}'", duty.creation_timestamp).as_str()),
            ))
        }

        Duties::new(mapped_duties)
    }
}