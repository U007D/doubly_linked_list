use crate::consts::*;
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyList,
    ExistingLiveReferences(usize)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Error::EmptyList => String::from(msg::ERR_EMPTY_LIST),
            Error::ExistingLiveReferences(count) => format!("{} {} {}",
                                                            msg::ERR_EXISTING_LIVE_REFERENCES_1,
                                                            count,
                                                            msg::ERR_EXISTING_LIVE_REFERENCES_2),
        })
    }
}
