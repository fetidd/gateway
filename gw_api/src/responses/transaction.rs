use serde::Serialize;

use super::{billing::BillingResponse, payment::PaymentResponse};

#[derive(Serialize, Default)]
pub struct TransactionResponse {
    pub baseamount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PaymentResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingResponse>,
    pub result: String,
}
