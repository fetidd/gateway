use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Default, Debug)]
pub struct CustomerRequest {
    first_name: Option<String>,
    last_name: Option<String>,
    premise: Option<String>,
    street: Option<String>,
    city: Option<String>,
    county: Option<String>,
    country: Option<String>,
}
