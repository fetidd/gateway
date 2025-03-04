use gw_core::card_scheme::CardScheme;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Default)]
pub struct PaymentRequest {
    payment_type: String,
    scheme: Option<CardScheme>,
    pan: Option<String>,
    security_code: Option<String>,
    expiry_month: Option<u8>,
    expiry_year: Option<u32>,
    account_number: Option<String>,
    sort_code: Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn deserialize_card() {
        let payment_json = r#"{"payment_type": "CARD", "scheme": "VISA", "pan": "4000111122223333", "security_code": "123", "expiry_month": 1, "expiry_year": 2021}"#;
        let actual: PaymentRequest = serde_json::from_str(payment_json).expect("deserialize payment request");
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
        let payment_json = r#"{"payment_type": "ACCOUNT", "account_number": "12341234", "sort_code": "123456"}"#;
        let actual: PaymentRequest = serde_json::from_str(payment_json).expect("deserialize payment request");
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
        let actual = serde_json::from_str::<PaymentRequest>(payment_json).unwrap_err().to_string();
        let expected = "missing field `payment_type` at line 1 column 53";
        assert_eq!(&actual, expected);
    }
}