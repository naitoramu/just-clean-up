use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CleaningPlanDto {

    title: String,

    address: String,

    cleaner_ids: Vec<String>,

    duties: Vec<DutyDto>,

    start_date: u64
}

#[derive(Serialize, Deserialize)]
pub struct DutyDto {

    pub title: String,

    pub description: String,

    #[serde(skip_deserializing)]
    pub img_src: Path,

    pub repetition: String,

    pub offset: String,

    pub penalty: String
}
