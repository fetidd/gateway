mod billing;
mod payment;
mod customer;
mod transaction_option;

use billing::BillingRequest;
use customer::CustomerRequest;
use payment::PaymentRequest;
use serde::Deserialize;
use transaction_option::TransactionOptionRequest;

#[derive(Deserialize, Default, Debug)]
pub struct TransactionRequest {
    pub baseamount: u32,
    pub merchant_id: String,
    pub payment: Option<PaymentRequest>,
    pub billing: Option<BillingRequest>,
    pub customer: Option<CustomerRequest>,
    pub options: Option<TransactionOptionRequest>,
}
