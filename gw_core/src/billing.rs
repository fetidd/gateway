use crate::country::Country;

pub struct Billing {
    first_name: String,
    last_name: String,
    premise: String,
    street: String,
    city: String,
    county: String,
    country: Country,
}
