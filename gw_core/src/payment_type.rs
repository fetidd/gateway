use crate::card_scheme::CardScheme;

pub type ExpiryDate = (u32, u8);

#[derive(Clone, Debug, PartialEq)]
pub enum PaymentType {
    Card { scheme: CardScheme, expiry_date: ExpiryDate, security_code: String },
    Account,
}