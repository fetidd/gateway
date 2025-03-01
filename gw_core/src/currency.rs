use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Currency {
    #[default]
    GBP,
    EUR,
    USD,
    JPY,
}

impl Currency {
    pub fn get_decimal_places(&self) -> usize {
        match self {
            Currency::GBP | Currency::EUR | Currency::USD => 2,
            Currency::JPY => 0,
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Currency::GBP => "GBP",
                Currency::EUR => "EUR",
                Currency::USD => "USD",
                Currency::JPY => "JPY",
            }
        )
    }
}
