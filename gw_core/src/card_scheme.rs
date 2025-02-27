use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum CardScheme {
    #[serde(rename = "VISA")]
    Visa,
    #[serde(rename = "MASTERCARD")]
    Mastercard,
}


impl std::fmt::Display for CardScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardScheme::Visa => "VISA",
            CardScheme::Mastercard => "MASTERCARD",
        };
        write!(f, "{s}")
    }
}