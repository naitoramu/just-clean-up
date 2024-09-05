use crate::domain::model::domain_model::DomainModel;
use crate::domain::model::routines::Routines;
use chrono::{DateTime, Utc};

#[derive(Clone)]
#[non_exhaustive]
pub struct CleaningPlan {

    pub id: String,

    pub title: String,

    pub address: String,

    pub participant_ids: Vec<String>,

    pub routines: Routines,

    pub start_date: DateTime<Utc>,

    pub status: CleaningPlanStatus,
}

impl CleaningPlan {
    pub fn new(
        id: String,
        title: String,
        address: String,
        participant_ids: Vec<String>,
        routines: Routines,
        start_date: DateTime<Utc>,
        status: CleaningPlanStatus,
    ) -> Self {
        CleaningPlan { id, title, address, participant_ids, routines, start_date, status }
    }
}

impl DomainModel for CleaningPlan {
    fn get_resource_name() -> &'static str {
        "Cleaning Plan"
    }
}

#[derive(Clone)]
pub enum  CleaningPlanStatus {
    PendingDutyAssignment,
    PendingFulfilment,
    Fulfilled,
    Scheduled
}

impl CleaningPlanStatus {

    pub fn to_string(&self) -> String {
        match self {
            CleaningPlanStatus::PendingDutyAssignment => "PENDING_DUTY_ASSIGNMENT",
            CleaningPlanStatus::PendingFulfilment => "PENDING_FULFILMENT",
            CleaningPlanStatus::Fulfilled => "FULFILLED",
            CleaningPlanStatus::Scheduled => "SCHEDULED"
        }.to_string()
    }

    pub fn from_string(status: String) -> Self {
        match status.as_str() {
           "PENDING_DUTY_ASSIGNMENT" => CleaningPlanStatus::PendingDutyAssignment,
           "PENDING_FULFILMENT" => CleaningPlanStatus::PendingFulfilment,
           "FULFILLED" => CleaningPlanStatus::Fulfilled,
           "SCHEDULED" => CleaningPlanStatus::Scheduled,
            _ => panic!("Invalid cleaning plan status: '{status}'")
        }
    }
}