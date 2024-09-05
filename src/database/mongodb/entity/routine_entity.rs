use serde::{Deserialize, Serialize};
use crate::database::mongodb::entity::duty_entity::DutyEntity;

#[derive(Serialize, Deserialize, Clone)]
pub struct RoutineEntity {

    pub repetition: String,

    pub offset: String,

    pub duties: Vec<DutyEntity>,
}