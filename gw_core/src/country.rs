use std::str::FromStr;

use serde::Serialize;

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
    fn try_from(value: String) -> Result<Country, String> {
        match value.as_str() {
            "GB" => Ok(Self::GB),
            "US" => Ok(Self::US),
            invalid => Err(format!("{invalid} is not a valid country code")),
        }
    }

    type Error = String;
}

impl FromStr for Country {
    type Err = String; // TODO make proper core error types

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GB" => Ok(Self::GB),
            "US" => Ok(Self::US),
            invalid => Err(format!("{invalid} is not a recognised country code")),
        }
    }
}
