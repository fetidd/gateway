mod billing;
mod payment;

use billing::BillingRequest;
use payment::PaymentRequest;
use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct TransactionRequest {
    pub baseamount: u32,
    pub payment: Option<PaymentRequest>,
    pub _billing: Option<BillingRequest>,
}
