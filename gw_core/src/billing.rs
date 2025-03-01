use crate::country::Country;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Billing {
    pub first_name: String,
    pub last_name: String,
    pub premise: String,
    pub street: String,
    pub city: String,
    pub county: String,
    pub country: Country,
}
