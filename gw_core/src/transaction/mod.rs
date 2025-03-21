pub mod transaction_builder;

use serde::{Deserialize, Serialize};
use validify::{schema_validation, ValidationErrors, Validify};

use crate::{
    account::Account, amount::Amount, billing::Billing, currency::Currency, customer::Customer,
    merchant::Merchant, payment::Payment,
};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Validify)]
#[validate(validate_transaction)]
pub struct Transaction {
    pub reference: String,
    pub r#type: TransactionType,
    pub amount: Amount,
    pub payment: Payment,
    pub billing: Billing,
    pub merchant: Merchant,
    pub account: Account,
    pub customer: Option<Customer>,
    pub status: TransactionStatus,
    pub currency: Currency,
}

impl PartialEq<Transaction> for Transaction {
    fn eq(&self, other: &Transaction) -> bool {
        self.r#type == other.r#type
            && self.amount == other.amount
            && self.payment == other.payment
            && self.billing == other.billing
            && self.merchant == other.merchant
            && self.customer == other.customer
            && self.status == other.status
            && self.account == other.account
            && self.currency == other.currency
    }
}

#[schema_validation]
fn validate_transaction(_t: &Transaction) -> Result<(), ValidationErrors> {}

#[cfg(test)]
mod tests {

    use rstest::*;

    #[rstest]
    fn test_schema_validate_transaction() {
        // let t = {
        //     let t = TransactionBuilder::new()
        //         .amount(12345)
        //         .payment(payment)
        //         .billing(billing)

        //     t.build()
        // };
    }
}
