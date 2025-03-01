use crate::{
    account::Account, amount::Amount, billing::Billing, customer::Customer, merchant::Merchant,
    payment::Payment,
};

pub enum Transaction {
    Auth {
        amount: Amount,
        payment: Payment,
        billing: Billing,
        merchant: Merchant,
        account: Account,
        customer: Customer,
    },
    Refund {
        amount: Amount,
        payment: Payment,
        billing: Billing,
        merchant: Merchant,
        account: Account,
        customer: Customer,
    },
    Query {},
}
