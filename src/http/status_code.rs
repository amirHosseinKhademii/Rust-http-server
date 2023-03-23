use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StatusCode {
    OK = 200,
    NotFound = 404,
    BadRequest = 400,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match self {
            Self::OK => "OK",
            Self::NotFound => "Not Found",
            Self::BadRequest => "Bad Request",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
