use crate::domain::model::duty::Duties;
use crate::domain::model::time_duration::TimeDuration;

#[non_exhaustive]
#[derive(Clone)]
pub struct Routines {
    routines: Vec<Routine>,
}

impl Routines {

    pub fn new(routines: Vec<Routine>) -> Self {
        Routines { routines }
    }

    pub fn vec(self) -> Vec<Routine> {
        self.routines
    }
}

#[non_exhaustive]
#[derive(Clone)]
pub struct Routine {
    pub repetition: TimeDuration,

    pub offset: TimeDuration,

    pub duties: Duties,
}

impl Routine {

    pub fn new(repetition: TimeDuration, offset: TimeDuration, duties: Duties) -> Self {
        Routine { repetition, offset, duties }
    }
}