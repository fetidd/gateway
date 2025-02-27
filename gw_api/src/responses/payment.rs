use gw_core::payment_type::PaymentType;
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
            PaymentType::Card { scheme } => {
                state.serialize_field("type", "CARD")?;
                state.serialize_field("scheme", &scheme.to_string())?;
            }
            PaymentType::Account => {
                state.serialize_field("type", "ACCOUNT")?;
            }
        }
        state.serialize_field("account_number", &self.account_number)?;
        state.end()
    }
}
