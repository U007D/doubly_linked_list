mod msg {
    pub const ERR_IO_ERROR: &str = "IoError encountered";
}

use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    io::Error as IoError,
};

#[derive(Debug)]
pub enum Error {
    IoError(IoError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Error::IoError(err) => format!("{} source: {:?}", msg::ERR_IO_ERROR, err),
        })
    }
}
