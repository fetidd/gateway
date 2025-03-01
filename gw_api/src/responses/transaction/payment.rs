use gw_core::{card_scheme::CardScheme, payment::Payment, utils};
use serde::Serialize;

#[derive(Serialize, Default, PartialEq, Debug)]
pub struct PaymentResponse<'a> {
    pub r#type: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme: Option<CardScheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_year: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pan: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sort_code: Option<&'a str>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub security_code: Option<&'a str>,
}

impl<'a> From<&Payment> for PaymentResponse<'a> {
    fn from(value: &Payment) -> Self {
        match value {
            Payment::Card {
                scheme,
                expiry_date,
                pan,
                ..
            } => Self {
                r#type: "CARD",
                scheme: Some(*scheme),
                expiry_month: Some(expiry_date.1),
                expiry_year: Some(expiry_date.0),
                pan: Some(utils::mask_pan(pan)),
                ..Default::default()
            },
            Payment::Account { account_number, .. } => Self {
                r#type: "ACCOUNT",
                account_number: Some(utils::mask_account_number(account_number)),
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn payment_card_into_response() {
        let payment = Payment::Card {
            scheme: CardScheme::Visa,
            expiry_date: (2023, 12),
            security_code: "123".into(),
            pan: "4000001111111234".into(),
        };
        let res: PaymentResponse = (&payment).into();
        let exp = PaymentResponse {
            r#type: "CARD",
            scheme: Some(CardScheme::Visa),
            expiry_month: Some(12),
            expiry_year: Some(2023),
            pan: Some("400000######1234".into()),
            ..Default::default()
        };
        assert_eq!(res, exp);
        let res = serde_json::to_string(&res).unwrap();
        let exp = "{\"type\":\"CARD\",\"scheme\":\"VISA\",\"expiry_month\":12,\"expiry_year\":2023,\"pan\":\"400000######1234\"}";
        assert_eq!(res, exp);
    }

    #[test]
    fn payment_account_into_response() {
        let payment = Payment::Account {
            account_number: "12341234".into(),
            sort_code: "010203".into(),
        };
        let res: PaymentResponse = (&payment).into();
        let exp = PaymentResponse {
            r#type: "ACCOUNT",
            account_number: Some("####1234".into()),
            ..Default::default()
        };
        assert_eq!(res, exp);
        let res = serde_json::to_string(&res).unwrap();
        let exp = "{\"type\":\"ACCOUNT\",\"account_number\":\"####1234\"}";
        assert_eq!(res, exp);
    }
}
