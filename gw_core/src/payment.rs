use crate::card_scheme::CardScheme;

pub type ExpiryDate = (u32, u8);

#[derive(Clone, Debug, PartialEq)]
pub enum Payment {
    Card {
        scheme: CardScheme,
        expiry_date: ExpiryDate,
        security_code: String,
        pan: String,
    },
    Account {
        account_number: String,
        sort_code: String,
    },
}
