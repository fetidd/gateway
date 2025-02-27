use serde::Deserialize;

use super::{billing::BillingRequest, payment::PaymentRequest};

#[derive(Deserialize, Default, Debug)]
pub struct TransactionRequest {
    pub baseamount: u32,
    pub payment: Option<PaymentRequest>,
    pub billing: Option<BillingRequest>,
}