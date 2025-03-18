pub mod billing;
pub mod customer;
pub mod payment;
pub mod transaction_option;

use billing::BillingRequest;
use customer::CustomerRequest;
use gw_core::{currency::Currency, transaction::TransactionType};
use payment::PaymentRequest;
use serde::Deserialize;
use transaction_option::TransactionOptionRequest;

#[derive(Deserialize, Debug)]
pub struct TransactionRequest {
    pub amount: u64,
    pub currency: Currency,
    pub transaction_type: TransactionType,
    pub merchant_id: String,
    pub payment: Option<PaymentRequest>,
    pub billing: Option<BillingRequest>,
    pub customer: Option<CustomerRequest>,
    pub options: Option<TransactionOptionRequest>,
}

impl TransactionRequest {
    pub fn take_payment_data(&mut self) -> Option<PaymentRequest> {
        self.payment.take()
    }
    pub fn take_billing_data(&mut self) -> Option<BillingRequest> {
        self.billing.take()
    }
    // pub fn take_customer_data(&mut self) -> Option<CustomerRequest> {
    //     self.customer.take()
    // }
    // pub fn take_options_data(&mut self) -> Option<OptionsRequest> {
    //     self.options.take()
    // }
}
