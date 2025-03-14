use serde::Serialize;
use crate::error::{Error, ErrorKind};

#[derive(Serialize, Clone, Copy, PartialEq, Debug, Default)]
pub enum Country {
    #[default]
    GB,
    US,
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Country::GB => "GB",
            Country::US => "US",
        };
        write!(f, "{c}")
    }
}

impl TryFrom<String> for Country {
    type Error = Error;

    fn try_from(value: String) -> Result<Country, Self::Error> {
        match value.as_str() {
            "GB" => Ok(Self::GB),
            "US" => Ok(Self::US),
            invalid => Err(Error {kind: ErrorKind::TypeError, message: format!("{invalid} is not a recognised country code"), source: None }),
        }
    }
}
