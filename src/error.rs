use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ResourceParseError {
    JsonParseError(serde_json::error::Error),
    UnknownFHIRResource,
    ValidationError,
}

impl fmt::Display for ResourceParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing the resource")
    }
}

impl Error for ResourceParseError {}

impl From<serde_json::error::Error> for ResourceParseError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::JsonParseError(err)
    }
}