mod billing;
mod payment;

use billing::BillingResponse;
use gw_core::{
    currency::Currency,
    transaction::{Transaction, TransactionError, TransactionStatus},
};
use payment::PaymentResponse;
use serde::Serialize;

#[derive(Default, PartialEq, Serialize, Debug)]
pub struct TransactionResponse<'a> {
    pub amount: u64, // TODO type alias this?
    pub currency: Currency,
    pub payment: PaymentResponse<'a>,
    pub billing: BillingResponse<'a>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TransactionError>,
    pub reference: String,
}

impl<'a> From<&'a Transaction> for TransactionResponse<'a> {
    fn from(value: &'a Transaction) -> Self {
        let error = if let TransactionStatus::Failed(e) = &value.status {
            Some(e)
        } else {
            None
        };

        Self {
            amount: value.amount.value(),
            currency: value.amount.currency(),
            payment: (&value.payment).into(),
            billing: (&value.billing).into(),
            status: value.status.to_string(),
            error: if let Some(e) = error { e.clone() } else { None },
            reference: value.reference.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::check_serialize_to_response;

    use super::*;

    use gw_core::{
        account::{Account, BankOneAccount},
        billing::Billing,
        card_scheme::CardScheme,
        currency::Currency,
        merchant::Merchant,
        payment::Payment,
        transaction::{transaction_builder::TransactionBuilder, TransactionType},
    };
    use rstest::*;

    #[rstest]
    fn can_serialise_to_response() {
        let card = Payment::Card {
            scheme: CardScheme::Visa,
            expiry_date: (2023, 1),
            security_code: "123".into(),
            pan: "4000111122223333".into(),
        };
        let acct = Account::BankOne(BankOneAccount {
            merchant_identification_value: "12345678".into(),
        });
        let mer = Merchant::default();
        let trx = TransactionBuilder::new()
            .transaction_type(TransactionType::Auth)
            .payment(card)
            .amount(12345)
            .currency(Currency::GBP)
            .account(acct)
            .merchant(mer)
            .billing(Billing::default())
            .build();
        let exp = TransactionResponse {
            amount: 12345,
            currency: Currency::GBP,
            payment: PaymentResponse {
                r#type: "CARD",
                scheme: Some(CardScheme::Visa),
                account_number: None,
                expiry_month: Some(1),
                expiry_year: Some(2023),
                pan: Some("400011######3333".into()),
            },
            billing: BillingResponse::default(),
            status: "SUCCESS".into(),
            error: None,
            reference: trx.reference.clone(),
        };
        let exp_json = r#"\{
  "amount": 12345,
  "currency": "GBP",
  "payment": \{
    "type": "CARD",
    "scheme": "VISA",
    "expiry_month": 1,
    "expiry_year": 2023,
    "pan": "400011######3333"
  \},
  "billing": \{
    "country": "GB"
  \},
  "status": "SUCCESS",
  "reference": "[0-9a-z-]+"
\}"#;
        check_serialize_to_response(&trx, &exp, exp_json);
    }
}
