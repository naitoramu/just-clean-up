use crate::error::json_problem::JsonProblem;
use crate::error::json_problems::JsonProblems;
use chrono::TimeDelta;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\d+)([smhdw])$").unwrap();
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct TimeDuration {
    pub time_delta: TimeDelta
}

impl TimeDuration {

    pub fn new(time_delta: TimeDelta) -> Self {
        TimeDuration { time_delta }
    }

    pub fn from_str(value: String) -> Result<Self, JsonProblem> {
        let lowercased = value.to_lowercase();
        let (count_match, unit_match) = match REGEX.captures(lowercased.as_str()) {
            Some(captures) => (captures.get(1).unwrap(), captures.get(2).unwrap()),
            None => return Err(JsonProblems::internal_server_error(format!("Invalid time duration '{lowercased}'").into()))
        };

        let count: i64 = count_match.as_str().parse().unwrap();

        let time_delta = match unit_match.as_str() {
            "s" => TimeDelta::seconds(count),
            "m" => TimeDelta::minutes(count),
            "h" => TimeDelta::hours(count),
            "d" => TimeDelta::days(count),
            "w" => TimeDelta::weeks(count),
            _ => return Err(JsonProblems::internal_server_error("Invalid time duration unit".into()))
        };

        Ok(Self::new(time_delta))
    }
}

impl Display for TimeDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.time_delta.num_weeks() > 0 {
            write!(f, "{}W", self.time_delta.num_weeks())
        } else if self.time_delta.num_days() > 0 {
            write!(f, "{}D", self.time_delta.num_days())
        } else if self.time_delta.num_hours() > 0 {
            write!(f, "{}H", self.time_delta.num_hours())
        } else if self.time_delta.num_minutes() > 0 {
            write!(f, "{}M", self.time_delta.num_minutes())
        } else {
            write!(f, "{}S", self.time_delta.num_seconds())
        }
    }
}