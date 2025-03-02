pub mod transaction_builder;

use crate::{
    account::Account, amount::Amount, billing::Billing, customer::Customer, merchant::Merchant,
    payment::Payment,
};

#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
    Auth,
    Refund,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    r#type: TransactionType,
    amount: Amount,
    payment: Payment,
    billing: Billing,
    merchant: Merchant,
    account: Account,
    customer: Option<Customer>,
}
