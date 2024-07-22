use serde::{Deserialize, Serialize};

use crate::entities::penalty::Penalty;

#[derive(Serialize, Deserialize)]
pub struct Duty {

    pub title: String,

    pub description: String,

    pub img_src: Option<String>,

    pub repetition: String,

    pub offset: String,

    pub penalty: Penalty
}

impl Duty {

    pub fn new(
        title: String,
        description: String,
        img_src: Option<String>,
        repetition: String,
        offset: String,
        penalty: Penalty
    ) -> Self {
        Duty { title, description, img_src, repetition, offset, penalty }
    }
}