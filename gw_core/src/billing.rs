use crate::country::Country;
use validify::Validify;

#[derive(Default, PartialEq, Debug, Clone, Validify)]
pub struct Billing {
    #[modify(trim)]
    pub first_name: String,
    #[modify(trim)]
    pub last_name: String,
    #[modify(trim)]
    pub premise: String,
    #[modify(trim)]
    pub street: String,
    #[modify(trim)]
    pub city: String,
    #[modify(trim)]
    pub county: String,
    pub country: Country,
}
