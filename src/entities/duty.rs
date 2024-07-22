use std::path::Path;
use crate::entities::penalty::Penalty;

pub struct Duty {

    pub title: String,

    pub description: String,

    pub img_src: Path,

    pub repetition: String,

    pub offset: String,

    pub penalty: Penalty
}