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
