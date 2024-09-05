#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct DutyFulfilment {
    pub completion_marked: bool,
    pub completion_confirmed: bool,
}

impl DutyFulfilment {

    pub fn new(marked: bool, confirmed: bool) -> Self {
        Self {
            completion_marked: marked,
            completion_confirmed: confirmed,
        }
    }
}