use crate::{
    account::Account,
    amount::{Amount, BASE},
    billing::Billing,
    customer::Customer,
    merchant::Merchant,
    payment::Payment,
};

pub enum Transaction {
    Auth {
        amount: Amount<BASE>,
        payment: Payment,
        billing: Billing,
        merchant: Merchant,
        account: Account,
        customer: Customer,
    },
    Refund {
        amount: Amount<BASE>,
        payment: Payment,
        billing: Billing,
        merchant: Merchant,
        account: Account,
        customer: Customer,
    },
    Query {},
}
