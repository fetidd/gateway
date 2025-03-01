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

/// Anything optional here but not in Transaction will be required when `build` is called.
#[derive(Default)]
struct TransactionBuilder {
    r#type: Option<TransactionType>,
    amount: Option<Amount<BASE>>,
    payment: Option<Payment>,
    billing: Option<Billing>,
    merchant: Option<Merchant>,
    account: Option<Account>,
    customer: Option<Customer>,
}

pub fn new_auth() -> TransactionBuilder {
    TransactionBuilder {
        r#type: Some(TransactionType::Auth),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::card_scheme::CardScheme;

    use super::*;
    use rstest::*;

    #[rstest]
    fn can_serialise_to_response() {
        let acct = Account::BankA {};
        let trx = new_auth()
            .amount(12345)
            .card(CardScheme::Visa, "4000111122223333", (2021, 3), "123")
            .account(&acct)
            .merchant(&mer)
            .build()
            .expect("build merchant");
    }
}
