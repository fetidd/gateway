use gw_core::{card_scheme::CardScheme, payment::Payment};
use serde::Deserialize;

use crate::error::{ErrorKind::Validation, GatewayError};

#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct PaymentRequest {
    payment_type: String,
    scheme: Option<CardScheme>,
    pan: Option<String>,
    security_code: Option<String>,
    expiry_month: Option<u8>,
    expiry_year: Option<u32>,
    account_number: Option<String>,
    sort_code: Option<String>,
}

impl PaymentRequest {
    fn get_card_missing(&self) -> Vec<&'static str> {
        let mut missing = vec![];
        if self.scheme.is_none() {
            missing.push("scheme");
        }
        if self.pan.is_none() {
            missing.push("pan");
        }
        if self.expiry_month.is_none() {
            missing.push("expiry_month");
        }
        if self.expiry_year.is_none() {
            missing.push("expiry_year");
        }
        if self.security_code.is_none() {
            missing.push("security_code");
        }
        missing
    }

    fn get_account_missing(&self) -> Vec<&'static str> {
        let mut missing = vec![];
        if self.sort_code.is_none() {
            missing.push("scheme");
        }
        if self.account_number.is_none() {
            missing.push("pan");
        }
        missing
    }
}

fn create_missing_error<T>(missing: &[&'static str]) -> Result<T, GatewayError> {
    let message = format!("Missing fields: {}", missing.join(", "));
    Err(GatewayError {
        kind: Validation,
        message,
    })
}

impl TryInto<Payment> for PaymentRequest {
    type Error = GatewayError;

    fn try_into(self) -> Result<Payment, Self::Error> {
        match self.payment_type.as_str() {
            "CARD" => {
                let missing = self.get_card_missing();
                if missing.len() > 0 {
                    return create_missing_error(&missing);
                }
                Ok(Payment::Card {
                    scheme: self.scheme.unwrap(),
                    expiry_date: (self.expiry_year.unwrap(), self.expiry_month.unwrap()),
                    security_code: self.security_code.unwrap(),
                    pan: self.pan.unwrap(),
                })
            }
            "ACCOUNT" => {
                let missing = self.get_account_missing();
                if missing.len() > 0 {
                    return create_missing_error(&missing);
                }
                Ok(Payment::Account {
                    account_number: self.account_number.unwrap(),
                    sort_code: self.sort_code.unwrap(),
                })
            }
            invalid => {
                return Err(GatewayError {
                    kind: Validation,
                    message: format!("{} is not a valid payment type", invalid),
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn deserialize_card() {
        let payment_json = r#"{"payment_type": "CARD", "scheme": "VISA", "pan": "4000111122223333", "security_code": "123", "expiry_month": 1, "expiry_year": 2021}"#;
        let actual: PaymentRequest =
            serde_json::from_str(payment_json).expect("deserialize payment request");
        let expected = PaymentRequest {
            payment_type: "CARD".into(),
            scheme: Some(CardScheme::Visa),
            pan: Some("4000111122223333".into()),
            security_code: Some("123".into()),
            expiry_month: Some(1),
            expiry_year: Some(2021),
            ..Default::default()
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn deserialize_account() {
        let payment_json =
            r#"{"payment_type": "ACCOUNT", "account_number": "12341234", "sort_code": "123456"}"#;
        let actual: PaymentRequest =
            serde_json::from_str(payment_json).expect("deserialize payment request");
        let expected = PaymentRequest {
            payment_type: "ACCOUNT".into(),
            account_number: Some("12341234".into()),
            sort_code: Some("123456".into()),
            ..Default::default()
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    fn deserialize_but_no_payment_type() {
        let payment_json = r#"{"account_number": "12341234", "sort_code": "123456"}"#;
        let actual = serde_json::from_str::<PaymentRequest>(payment_json)
            .unwrap_err()
            .to_string();
        let expected = "missing field `payment_type` at line 1 column 53";
        assert_eq!(&actual, expected);
    }
}
