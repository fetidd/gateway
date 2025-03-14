use gw_core::billing::Billing;
use gw_core::error::Error;
use serde::Deserialize;

use crate::error::ErrorKind::Validation;
use crate::error::GatewayError;

#[derive(Deserialize, Default, Debug)]
pub struct BillingRequest {
    first_name: Option<String>,
    last_name: Option<String>,
    premise: Option<String>,
    street: Option<String>,
    city: Option<String>,
    county: Option<String>,
    country: Option<String>,
}

impl TryFrom<BillingRequest> for Billing {
    fn try_from(value: BillingRequest) -> Result<Self, GatewayError> {
        Ok(Billing {
            first_name: value.first_name.unwrap_or_default(),
            last_name: value.last_name.unwrap_or_default(),
            premise: value.premise.unwrap_or_default(),
            street: value.street.unwrap_or_default(),
            city: value.city.unwrap_or_default(),
            county: value.county.unwrap_or_default(),
            country: value
                .country
                .unwrap_or_default()
                .try_into()
                .map_err(|e: Error| GatewayError {
                    kind: Validation,
                    message: e.to_string(),
                })?,
        })
    }

    type Error = GatewayError;
}
