mod billing;
mod customer;
mod payment;
mod transaction_option;

use billing::BillingRequest;
use customer::CustomerRequest;
use gw_core::transaction::TransactionType;
use payment::PaymentRequest;
use serde::Deserialize;
use transaction_option::TransactionOptionRequest;

#[derive(Deserialize, Debug)]
pub struct TransactionRequest {
    pub amount: u64,
    pub transaction_type: TransactionType,
    pub merchant_id: String,
    pub payment: Option<PaymentRequest>,
    pub billing: Option<BillingRequest>,
    pub customer: Option<CustomerRequest>,
    pub options: Option<TransactionOptionRequest>,
}
