use gw_core::{billing::Billing, country::Country};
use serde::Serialize;

#[derive(Serialize, Default, PartialEq, Clone, Copy, Debug)]
pub struct BillingResponse<'a> {
    #[serde(skip_serializing_if = "str::is_empty")]
    first_name: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    last_name: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    premise: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    street: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    city: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    county: &'a str,
    // #[serde(skip_serializing_if = "Option::is_none")]
    country: Country,
}

impl<'a> From<&'a Billing> for BillingResponse<'a> {
    fn from(value: &'a Billing) -> Self {
        Self {
            first_name: &value.first_name,
            last_name: &value.last_name,
            premise: &value.premise,
            street: &value.street,
            city: &value.city,
            county: &value.county,
            country: value.country,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::*;

    use super::*;
    use gw_core::billing::Billing;

    #[test]
    fn can_serialize_from_billing() {
        let mut billing = Billing::default();
        let exp = BillingResponse::default();
        let exp_json = r#"{
  "country": "GB"
}"#;
        check_serialize_to_response(&billing, &exp, exp_json);

        billing.first_name = "Ben".into();
        billing.last_name = "Jones".into();
        billing.city = "Llandudno Junction".into();
        let exp = BillingResponse {
            first_name: "Ben",
            last_name: "Jones",
            city: "Llandudno Junction",
            country: Country::GB,
            ..Default::default()
        };
        let exp_json = r#"{
  "first_name": "Ben",
  "last_name": "Jones",
  "city": "Llandudno Junction",
  "country": "GB"
}"#;
        check_serialize_to_response(&billing, &exp, exp_json);
    }
}
