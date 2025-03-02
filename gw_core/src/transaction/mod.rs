pub mod transaction_builder;

use serde::Serialize;

use crate::{
    account::Account, amount::Amount, billing::Billing, customer::Customer, merchant::Merchant,
    payment::Payment,
};

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum TransactionType {
    Auth,
    Refund,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub enum TransactionStatus {
    #[default]
    Success,
    Failed(Option<TransactionError>),
}

impl std::fmt::Display for TransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let d = match self {
            TransactionStatus::Success => "SUCCESS",
            TransactionStatus::Failed(_) => "FAILED",
        };
        write!(f, "{d}")
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TransactionError {}

#[derive(Debug, PartialEq, Clone)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub amount: Amount,
    pub payment: Payment,
    pub billing: Billing,
    pub merchant: Merchant,
    pub account: Account,
    pub customer: Option<Customer>,
    pub status: TransactionStatus,
}
