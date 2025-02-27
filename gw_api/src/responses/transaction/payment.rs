use gw_core::{payment_type::PaymentType, utils};
use serde::{ser::SerializeStruct, Serialize};

pub struct PaymentResponse {
    pub payment_type: PaymentType,
    pub account_number: String,
}

impl Serialize for PaymentResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PaymentResponse", 2)?;
        match &self.payment_type {
            PaymentType::Card {
                scheme,
                expiry_date,
                security_code: _,
            } => {
                state.serialize_field("type", "CARD")?;
                state.serialize_field("scheme", &scheme.to_string())?;
                state.serialize_field("expiry_year", &expiry_date.0)?;
                state.serialize_field("expiry_month", &expiry_date.1)?;
            }
            PaymentType::Account => {
                state.serialize_field("type", "ACCOUNT")?;
            }
        }
        state.serialize_field("account_number", &self.account_number)?;
        state.end()
    }
}
