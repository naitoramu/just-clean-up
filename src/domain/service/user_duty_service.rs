use crate::database::crud_repository::CrudRepository;
use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
use crate::domain::model::duty::Duty;
use crate::domain::model::duty_fulfilment::DutyFulfilment;
use crate::domain::model::penalty::UserPenalty;
use crate::domain::model::user_duty::UserDuty;
use crate::domain::model::user_tasks::UserTasks;
use crate::error::json_problem::JsonProblem;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;

pub struct UserDutyService {
    cleaning_plan_repository: Arc<dyn CrudRepository<CleaningPlan> + Send + Sync>,
    user_duty_repository: Arc<dyn CrudRepository<UserDuty> + Send + Sync>,
}

impl UserDutyService {

    pub fn new(
        cleaning_plan_repository: Arc<dyn CrudRepository<CleaningPlan>>,
        user_duty_repository: Arc<dyn CrudRepository<UserDuty>>,
    ) -> Self {
        UserDutyService { cleaning_plan_repository, user_duty_repository }
    }

    pub async fn make_schedules(&self) -> Result<Vec<String>, JsonProblem> {
        let plans_to_schedule = self.cleaning_plan_repository.find_all_matching(HashMap::from([
            ("status".to_string(), CleaningPlanStatus::PendingDutyAssignment.to_string()),
        ])).await?;

        let mut created_user_duty_ids: Vec<String> = Vec::new();

        for mut plan in plans_to_schedule {
            created_user_duty_ids.append(&mut self.create_user_duties(plan.clone()).await?);
            plan.status = CleaningPlanStatus::Scheduled;
            self.cleaning_plan_repository.update(plan.id.clone(), &plan).await?;
        }

        Ok(created_user_duty_ids)
    }

    async fn create_user_duties(&self, cleaning_plan: CleaningPlan) -> Result<Vec<String>, JsonProblem> {
        let user_to_duty = self.assign_duties_to_users(cleaning_plan.participant_ids.clone(), &cleaning_plan.duties).await?;
        let mut created_duties_ids: Vec<String> = Vec::new();
        for (assigned_duty, user_id) in user_to_duty {
            let user_duty = UserDuty::new(
                "".to_string(),
                assigned_duty.id.clone(),
                user_id,
                assigned_duty.title.clone(),
                UserTasks::from_template(&assigned_duty.todo_list),
                Utc::now() + assigned_duty.repetition.time_delta - assigned_duty.offset.time_delta,
                Utc::now() + assigned_duty.repetition.time_delta + assigned_duty.offset.time_delta,
                DutyFulfilment::new(false, false),
                UserPenalty::new("".to_string(), assigned_duty.penalty.clone(), false),
            );
            created_duties_ids.push(self.user_duty_repository.create(&user_duty).await?.id);
        }
        Ok(created_duties_ids)
    }

    async fn assign_duties_to_users<'a>(&self, user_ids: Vec<String>, duties: &'a Vec<Duty>) -> Result<HashMap<&'a Duty, String>, JsonProblem> {
        let mut duty_to_user_id: HashMap<&Duty, String> = HashMap::new();
        let mut unassigned_user_ids: Vec<String> = user_ids.clone();

        for duty in duties {
            if unassigned_user_ids.is_empty() {
                unassigned_user_ids = user_ids.clone();
            }
            let user_id = self.get_user_longest_unassigned_to_duty(duty.id.clone(), &unassigned_user_ids).await?;
            let user_id_index = unassigned_user_ids.iter().position(|id| *id == user_id).unwrap();
            duty_to_user_id.insert(duty, user_id);
            unassigned_user_ids.remove(user_id_index);
        };

        Ok(duty_to_user_id)
    }

    async fn get_user_longest_unassigned_to_duty(&self, duty_id: String, user_ids: &Vec<String>) -> Result<String, JsonProblem> {
        let earliest_timestamp: DateTime<Utc> = Utc::now();
        let mut longest_unassigned_user_id = user_ids.get(0).unwrap();
        for user_id in user_ids {
            let timestamp: DateTime<Utc> = self.get_last_completed_by_user_duty_timestamp(duty_id.clone(), user_id.clone()).await?;
            if timestamp < earliest_timestamp { longest_unassigned_user_id = user_id }
        }

        Ok(longest_unassigned_user_id.clone())
    }

    async fn get_last_completed_by_user_duty_timestamp(&self, duty_id: String, user_id: String) -> Result<DateTime<Utc>, JsonProblem> {
        let user_duties = self.user_duty_repository.find_all_matching(HashMap::from([
            ("template_id".to_string(), duty_id),
            ("user_id".to_string(), user_id),
        ])).await?;

        let mut latest_timestamp = DateTime::from_timestamp(0, 0).unwrap();
        for duty in user_duties {
            if duty.start_time > latest_timestamp { latest_timestamp = duty.start_time }
        }
        Ok(latest_timestamp)
    }
}