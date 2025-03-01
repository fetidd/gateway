pub mod transaction_builder;

use crate::{
    account::Account,
    amount::{Amount, BASE},
    billing::Billing,
    customer::Customer,
    merchant::Merchant,
    payment::Payment,
};

pub enum TransactionType {
    Auth,
    Refund,
}

pub struct Transaction {
    r#type: TransactionType,
    amount: Amount<BASE>,
    payment: Payment,
    billing: Billing,
    merchant: Merchant,
    account: Account,
    customer: Option<Customer>,
}
