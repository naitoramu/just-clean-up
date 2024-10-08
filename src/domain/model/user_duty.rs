use chrono::{DateTime, Utc};
use crate::domain::model::domain_model::DomainModel;
use crate::domain::model::duty_fulfilment::DutyFulfilment;
use crate::domain::model::user_penalty::UserPenalty;
use crate::domain::model::user_tasks::UserTasks;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UserDuty {
    pub id: String,
    pub user_id: String,
    pub template_id: String,
    pub title: String,
    pub tasks: UserTasks,
    pub start_time: DateTime<Utc>,
    pub deadline_time: DateTime<Utc>,
    pub duty_fulfilment: DutyFulfilment,
    pub penalty: UserPenalty,
}

impl UserDuty {

    pub fn new(
        id: String,
        user_id: String,
        template_id: String,
        title: String,
        tasks: UserTasks,
        start_time: DateTime<Utc>,
        deadline_time: DateTime<Utc>,
        duty_fulfilment: DutyFulfilment,
        penalty: UserPenalty,
    ) -> Self {
        UserDuty {
            id,
            user_id,
            template_id,
            title,
            tasks,
            start_time,
            deadline_time,
            duty_fulfilment,
            penalty,
        }
    }
}

impl DomainModel for UserDuty {

    fn id(&self) -> String {
        self.id.clone()
    }

    fn get_resource_name() -> &'static str {
        "User Duty"
    }
}