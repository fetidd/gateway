use crate::card_scheme::CardScheme;

#[derive(Clone, Copy, Debug)]
pub enum PaymentType {
    Card { scheme: CardScheme },
    Account,
}