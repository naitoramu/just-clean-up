use chrono::DateTime;
use mongodb::bson::oid::ObjectId;
use crate::database::mongodb::entity::user_duty_entity::{UserDutyEntity, UserPenaltyEntity, UserTaskEntity};
use crate::database::mongodb::mapper::mapper::Mapper;
use crate::domain::model::duty_fulfilment::DutyFulfilment;
use crate::domain::model::penalty::UserPenalty;
use crate::domain::model::user_duty::UserDuty;
use crate::domain::model::user_tasks::{UserTask, UserTasks};
use crate::error::json_problem::JsonProblem;

pub struct UserDutyEntityMapper;

impl Mapper<UserDuty, UserDutyEntity> for UserDutyEntityMapper {
    fn map_to_entity(domain_model: UserDuty) -> Result<UserDutyEntity, JsonProblem> {
        Ok(
            UserDutyEntity {
                id: Self::str_to_object_id(&domain_model.id)?,
                template_id: Self::str_to_object_id(&domain_model.template_id)?,
                title: domain_model.title,
                tasks: Self::map_domain_model_tasks_to_entity(domain_model.tasks)?,
                start_timestamp: domain_model.start_time.timestamp(),
                deadline_timestamp: domain_model.deadline_time.timestamp(),
                completion_marked: domain_model.duty_fulfilment.completion_marked,
                completion_confirmed: domain_model.duty_fulfilment.completion_confirmed,
                penalty: UserPenaltyEntity {
                    id: Self::str_to_object_id(&domain_model.penalty.id)?,
                    content: domain_model.penalty.content,
                    fulfilled: domain_model.penalty.fulfilled,
                },
            }
        )
    }

    fn map_to_domain_model(entity: UserDutyEntity) -> UserDuty {
        UserDuty::new(
            entity.id.to_hex(),
            entity.template_id.to_hex(),
            entity.title,
            UserTasks::new(entity.tasks.iter().map(Self::map_entity_task_to_domain_task).collect()),
            DateTime::from_timestamp(entity.start_timestamp, 0)
                .expect(format!("Failed to parse timestamp '{}'", entity.start_timestamp).as_str()),
            DateTime::from_timestamp(entity.deadline_timestamp, 0)
                .expect(format!("Failed to parse timestamp '{}'", entity.deadline_timestamp).as_str()),
            DutyFulfilment::new(entity.completion_marked, entity.completion_confirmed),
            UserPenalty::new(
                entity.penalty.id.to_hex(),
                entity.penalty.content,
                entity.penalty.fulfilled
            )
        )
    }
}

impl UserDutyEntityMapper {

    fn map_domain_model_tasks_to_entity(tasks: UserTasks) -> Result<Vec<UserTaskEntity>, JsonProblem> {
        Ok(tasks.tasks.iter().map(|task| {
            Ok(UserTaskEntity {
                id: Self::str_to_object_id(&task.id)?,
                task: task.task.clone(),
                accepting_user_ids: task.accepting_user_ids.iter()
                    .map(Self::str_to_object_id)
                    .collect::<Result<Vec<ObjectId>, JsonProblem>>()?,
                rejecting_user_ids: task.rejecting_user_ids.iter()
                    .map(Self::str_to_object_id)
                    .collect::<Result<Vec<ObjectId>, JsonProblem>>()?,
            })
        }).collect::<Result<Vec<UserTaskEntity>, JsonProblem>>()?)
    }

    fn map_entity_task_to_domain_task(entity: &UserTaskEntity) -> UserTask {
        UserTask::new(
            entity.id.to_hex(),
            entity.task.clone(),
            entity.accepting_user_ids.iter().map(|user_id| user_id.to_hex()).collect(),
            entity.rejecting_user_ids.iter().map(|user_id| user_id.to_hex()).collect(),
        )
    }
}