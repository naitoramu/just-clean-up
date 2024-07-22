use crate::entities::duty::Duty;

pub struct CleaningPlan {

    title: String,

    address: String,

    cleaner_ids: Vec<String>,

    duties: Vec<Duty>,

    start_date: u64
}