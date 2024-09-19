use crate::database::cleaning_plan_repository::CleaningPlanRepository;
use crate::database::user_duty_repository::UserDutyRepository;
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::domain::model::duty::{Duties, Duty};
use crate::domain::model::duty_fulfilment::DutyFulfilment;
use crate::domain::model::user_duty::UserDuty;
use crate::domain::model::user_penalty::UserPenalty;
use crate::domain::model::user_tasks::UserTasks;
use crate::error::json_problem::JsonProblem;
use chrono::{DateTime, TimeDelta, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use log::{debug, trace};
use crate::domain::model::routines::Routine;

pub struct UserDutyService {
    cleaning_plan_repository: Arc<dyn CleaningPlanRepository + Send + Sync>,
    user_duty_repository: Arc<dyn UserDutyRepository + Send + Sync>,
}

impl UserDutyService {
    pub fn new(
        cleaning_plan_repository: Arc<dyn CleaningPlanRepository + Send + Sync>,
        user_duty_repository: Arc<dyn UserDutyRepository + Send + Sync>,
    ) -> Self {
        UserDutyService { cleaning_plan_repository, user_duty_repository }
    }

    pub async fn get_all_user_duties(&self, user_id: String) -> Result<Vec<UserDuty>, JsonProblem> {
        // TODO: matching String and ObjectID in the database does not work, make better repositories implementation
        self.user_duty_repository.get_all_user_duties(user_id).await
    }

    pub async fn make_schedules(&self) -> Result<Vec<UserDuty>, JsonProblem> {
        let plans_to_schedule = self.cleaning_plan_repository.get_plans_with_status(
            CleaningPlanStatus::PendingDutyAssignment
        ).await?;

        let mut created_user_duties: Vec<UserDuty> = Vec::new();

        for mut plan in plans_to_schedule {
            created_user_duties.append(&mut self.create_user_duties(plan.clone()).await?);
            plan.status = CleaningPlanStatus::Scheduled;
            self.cleaning_plan_repository.update_plan(plan.id.clone(), &plan).await?;
        }

        Ok(created_user_duties)
    }

    async fn create_user_duties(&self, cleaning_plan: CleaningPlan) -> Result<Vec<UserDuty>, JsonProblem> {
        let mut created_duties: Vec<UserDuty> = Vec::new();

        for routine in cleaning_plan.routines.vec() {
            let duty_to_user = self.assign_routine_duties_to_users(cleaning_plan.participant_ids.clone(), routine.duties.clone()).await?;
            for (assigned_duty, user_id) in duty_to_user {
                let user_duty = Self::build_user_duty(user_id, assigned_duty, routine.clone());
                created_duties.push(self.user_duty_repository.create_user_duty(&user_duty).await?);
            }
        }

        Ok(created_duties)
    }

    async fn assign_routine_duties_to_users<'a>(&self, user_ids: Vec<String>, duties: Duties) -> Result<HashMap<Duty, String>, JsonProblem> {
        let mut duty_to_user_id: HashMap<Duty, String> = HashMap::new();
        let mut assigned_user_ids = Vec::new();

        for duty in duties.sort_by_creation_time().vec() {
            let selected_user_id = self.select_user_for_duty(duty.id.clone(), &user_ids, &assigned_user_ids).await?;
            duty_to_user_id.insert(duty, selected_user_id.clone());
            assigned_user_ids.push(selected_user_id);
            if assigned_user_ids.len() == user_ids.len() {
                assigned_user_ids.clear()
            }
        };

        Ok(duty_to_user_id)
    }

    async fn select_user_for_duty(&self, duty_id: String, user_ids: &Vec<String>, assigned_user_ids: &Vec<String>) -> Result<String, JsonProblem> {
        debug!("Selecting user for duty '{}'", duty_id.clone());
        let mut oldest_timestamp = Utc::now();
        let mut selected_user_id: Option<String> = None;

        for user_id in user_ids {
            trace!("Analysing user '{}'", user_id.clone());
            let timestamp = self.get_user_duty_completion_timestamp(duty_id.clone(), user_id.clone()).await?;
            if assigned_user_ids.contains(user_id) {
                continue;
            }
            trace!("User '{}' not in assigned users '{:?}'", user_id.clone(), assigned_user_ids);
            if let Some(timestamp) = timestamp {
                if timestamp < oldest_timestamp {
                    oldest_timestamp = timestamp;
                    selected_user_id = Some(user_id.clone());
                }
            } else {
                selected_user_id = Some(user_id.clone());
                break;
            }
        }

        Ok(selected_user_id.expect("Unable to select user for duty"))
    }

    async fn get_user_duty_completion_timestamp(&self, duty_id: String, user_id: String) -> Result<Option<DateTime<Utc>>, JsonProblem> {
        let user_duties = self.user_duty_repository.get_user_duties_by_duty_template(user_id.clone(), duty_id).await?;
        debug!("Fetched {} duties for user '{}': {:?}", user_duties.len(), user_id, user_duties);

        if user_duties.is_empty() {
            return Ok(None);
        }

        let mut latest_timestamp = DateTime::from_timestamp(0, 0).unwrap();
        for duty in user_duties {
            if duty.start_time > latest_timestamp { latest_timestamp = duty.start_time }
        }
        Ok(Some(latest_timestamp))
    }

    fn build_user_duty(user_id: String, assigned_duty: Duty, routine: Routine) -> UserDuty {
        UserDuty::new(
            "".to_string(),
            user_id,
            assigned_duty.id,
            assigned_duty.title,
            UserTasks::from_template(&assigned_duty.todo_list),
            Utc::now() + routine.repetition.time_delta - routine.offset.time_delta,
            Utc::now() + routine.repetition.time_delta + routine.offset.time_delta,
            DutyFulfilment::new(false, false),
            UserPenalty::new("".to_string(), assigned_duty.penalty, false),
        )
    }
}