mod billing;
mod payment;

use billing::BillingResponse;
use gw_core::amount::{Amount, DEC};
use payment::PaymentResponse;
use serde::Serialize;

#[derive(Serialize, Default)]
pub struct TransactionResponse<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseamount: Option<Amount<DEC>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PaymentResponse<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<BillingResponse<'a>>,
    pub result: String,
}
