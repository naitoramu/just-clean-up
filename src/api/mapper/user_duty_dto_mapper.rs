use crate::api::dto::user_duty_dto::{DutyFulfilmentDto, UserDutyDto, UserPenaltyDto, UserTaskDto};
use crate::api::mapper::dto_mapper::DtoMapper;
use crate::domain::model::duty_fulfilment::DutyFulfilment;
use crate::domain::model::user_duty::UserDuty;
use crate::domain::model::user_penalty::UserPenalty;
use crate::domain::model::user_tasks::{UserTask, UserTasks};
use chrono::DateTime;

pub trait UserDutyDtoMapper {}

impl DtoMapper<UserDutyDto, UserDuty> for dyn UserDutyDtoMapper {
    fn map_to_domain_model(dto: UserDutyDto) -> UserDuty {
        UserDuty::new(
            dto.id,
            dto.user_id,
            dto.template_id,
            dto.title,
            UserTasks::new(dto.tasks.iter().map(Self::map_dto_task_to_domain_task).collect()),
            DateTime::parse_from_rfc3339(dto.start_time.as_str()).unwrap().to_utc(),
            DateTime::parse_from_rfc3339(dto.deadline_time.as_str()).unwrap().to_utc(),
            DutyFulfilment::new(dto.duty_fulfilment.completion_marked, dto.duty_fulfilment.completion_confirmed),
            UserPenalty::new(
                dto.penalty.id,
                dto.penalty.content,
                dto.penalty.fulfilled,
            ),
        )
    }

    fn map_to_dto(domain_model: UserDuty) -> UserDutyDto {
        UserDutyDto {
            id: domain_model.id,
            user_id: domain_model.user_id,
            template_id: domain_model.template_id,
            title: domain_model.title,
            tasks: Self::map_domain_model_tasks_to_entity(domain_model.tasks),
            start_time: "".to_string(),
            deadline_time: "".to_string(),
            duty_fulfilment: DutyFulfilmentDto {
                completion_marked: domain_model.duty_fulfilment.completion_marked,
                completion_confirmed: domain_model.duty_fulfilment.completion_confirmed,
            },
            penalty: UserPenaltyDto {
                id: domain_model.penalty.id,
                content: domain_model.penalty.content,
                fulfilled: domain_model.penalty.fulfilled,
            },
        }
    }
}

impl dyn UserDutyDtoMapper {
    fn map_dto_task_to_domain_task(dto: &UserTaskDto) -> UserTask {
        UserTask::new(
            dto.id.clone(),
            dto.task.clone(),
            dto.accepting_user_ids.clone(),
            dto.rejecting_user_ids.clone(),
        )
    }

    fn map_domain_model_tasks_to_entity(domain_tasks: UserTasks) -> Vec<UserTaskDto> {
        domain_tasks.tasks.iter().map(|task| {
            UserTaskDto {
                id: task.id.clone(),
                task: task.task.clone(),
                accepting_user_ids: task.accepting_user_ids.clone(),
                rejecting_user_ids: task.rejecting_user_ids.clone(),
            }
        }).collect()
    }
}