mod billing;
mod payment;

use billing::BillingResponse;
use gw_core::amount::{Amount, DEC};
use payment::PaymentResponse;
use serde::Serialize;

#[derive(Default)]
pub struct TransactionResponse<'a> {
    pub amount: Option<Amount<DEC>>,
    pub payment: Option<PaymentResponse<'a>>,
    pub billing: Option<BillingResponse<'a>>,
    pub result: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gw_core::{
        card_scheme::CardScheme,
        transaction::{new_auth, Transaction},
    };
    use rstest::*;

    // #[rstest]
    // fn can_serialise_to_response() {
    //     let acct = Account::BankA {};
    //     let trx = new_auth()
    //         .amount(12345)
    //         .card(CardScheme::Visa, "4000111122223333", (2021, 3), "123")
    //         .account(&acct)
    //         .merchant(&mer)
    //         .build()
    //         .expect("build merchant");
    // }
}
