use gw_core::{card_scheme::CardScheme, payment_type::PaymentType};
use serde::{de::Visitor, Deserialize};
use tracing::{error, span, Level};

#[derive(Debug, PartialEq)]
pub struct PaymentRequest {
    pub payment_type: PaymentType,
    pub account_number: String,
}

impl<'de> Deserialize<'de> for PaymentRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let span = span!(Level::ERROR, "deserializing");
        let _guard = span.enter();
        
        // The fields we expect in the payment object, plus Ignore to allow us to deserialize objects with extra fields
        #[allow(non_camel_case_types)]
        enum Field {
            r#type, account_number, scheme, expiry_year, expiry_month, security_code//, ignore // allowing ignore causes a compiler warning "this function depends on never type fallback being `()`"
        }

        struct FieldVisitor;
        impl<'de> Visitor<'de> for FieldVisitor {
            type Value = Field;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "payment object field")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                match v {
                    "type" => Ok(Field::r#type),
                    "account_number" => Ok(Field::account_number),
                    "scheme" => Ok(Field::scheme),
                    "expiry_year" => Ok(Field::expiry_year),
                    "expiry_month" => Ok(Field::expiry_month),
                    "security_code" => Ok(Field::security_code),
                    f => {
                        error!(field=%f, "invalid field");
                        return Err(serde::de::Error::unknown_field(f, &["type", "account_number", "scheme", "expiry_year", "expiry_month", "security_code"]));
                    }
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de> {
                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PaymentRequestVisitor;
        impl<'de> Visitor<'de> for PaymentRequestVisitor {
            type Value = PaymentRequest;
        
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "payment request")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: serde::de::MapAccess<'de>, {
                let mut r#type: Option<&'de str> = None;
                let mut scheme: Option<&'de str> = None;
                let mut account_number: Option<&'de str> = None;
                let mut expiry_year: Option<u32> = None;
                let mut expiry_month: Option<u8> = None;
                let mut security_code: Option<&'de str> = None;
                while let Some(key) = map.next_key::<Field>()? {
                    match key {
                        Field::r#type => {
                            if r#type.is_some() {
                                error!("duplicate field: 'type'");
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = Some(map.next_value()?);
                        },
                        Field::account_number => {
                            if account_number.is_some() {
                                error!("duplicate field: 'account_number'");
                                return Err(serde::de::Error::duplicate_field("account_number"));
                            }
                            account_number = Some(map.next_value()?);
                        },
                        Field::scheme => {
                            if scheme.is_some() {
                                error!("duplicate field: 'scheme'");
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme = Some(map.next_value()?);
                        },
                        Field::expiry_year => {
                            if expiry_year.is_some() {
                                error!("duplicate field: 'expiry_year'");
                                return Err(serde::de::Error::duplicate_field("expiry_year"));
                            }
                            expiry_year = Some(map.next_value()?);
                        },
                        Field::expiry_month => {
                            if expiry_month.is_some() {
                                error!("duplicate field: 'expiry_month'");
                                return Err(serde::de::Error::duplicate_field("expiry_month"));
                            }
                            expiry_month = Some(map.next_value()?);
                        },
                        Field::security_code => {
                            if security_code.is_some() {
                                error!("duplicate field: 'security_code'");
                                return Err(serde::de::Error::duplicate_field("security_code"));
                            }
                            security_code = Some(map.next_value()?);
                        },
                        // _ => { // allowing ignore/unknown fields causes a compiler warning "this function depends on never type fallback being `()`"
                        //     let _ = map.next_value()?;
                        // }
                    }
                }

                let r#type = match r#type {
                    Some(t) => t,
                    None => {
                        error!("missing field: 'type'");
                        return Err(<A::Error as serde::de::Error>::missing_field("type"));
                    },
                };
                let account_number = match account_number {
                    Some(n) => n,
                    None => {
                        error!("missing field: 'account_number'");
                        return Err(<A::Error as serde::de::Error>::missing_field("account_number"));
                    },
                };
                match r#type {
                    "CARD" => {
                        let scheme = match scheme {
                            Some(s) => match s {
                                "VISA" => CardScheme::Visa,
                                "MASTERCARD" => CardScheme::Mastercard,
                                invalid => {
                                    error!(%invalid, "invalid value for field 'scheme'");
                                    return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(invalid), &"VISA, MASTERCARD"));
                                }
                            },
                            None => {
                                error!("missing field: 'scheme'");
                                return Err(<A::Error as serde::de::Error>::missing_field("scheme"));
                            },
                        };
                        let expiry_year = match expiry_year {
                            Some(year) => match year {
                                year => year
                            },
                            None => {
                                error!("missing field: 'expiry_year'");
                                return Err(<A::Error as serde::de::Error>::missing_field("expiry_year"));
                            },
                        };
                        let expiry_month = match expiry_month {
                            Some(month) => match month {
                                month => month
                            },
                            None => {
                                error!("missing field: 'expiry_month'");
                                return Err(<A::Error as serde::de::Error>::missing_field("expiry_month"));
                            },
                        };
                        let security_code = match security_code {
                            Some(month) => match month {
                                month => month
                            },
                            None => {
                                error!("missing field: 'security_code'");
                                return Err(<A::Error as serde::de::Error>::missing_field("security_code"));
                            },
                        };
                        Ok(Self::Value {payment_type: PaymentType::Card { scheme, expiry_date: (expiry_year, expiry_month), security_code: security_code.into() }, account_number: account_number.into()})
                    },
                    "ACCOUNT" => {
                        let mut invalids = vec![];
                        if expiry_month.is_some() { // TODO can this be dried up at all? Or is this just Rust being Rust?
                            invalids.push("expiry_month");
                        }
                        if expiry_year.is_some() {
                            invalids.push("expiry_year");
                        }
                        if security_code.is_some() {
                            invalids.push("security_code");
                        }
                        if scheme.is_some() {
                            invalids.push("scheme");
                        }
                        if invalids.len() > 0 {
                            let field = invalids.join(", ");
                            return Err(<A::Error as serde::de::Error>::unknown_field(&field, &["type", "account_number"]));
                        }
                        Ok(Self::Value {payment_type: PaymentType::Account, account_number: account_number.into()})
                    }
                    invalid => {
                        error!(%invalid, "invalid value for field 'scheme'");
                        return Err(serde::de::Error::invalid_value(serde::de::Unexpected::Str(invalid), &"CARD, ACCOUNT"));
                    }
                }
            }
        }
        deserializer.deserialize_struct("PaymentRequest", &["payment_type", "account_number"],PaymentRequestVisitor)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_deserialize_payment_request() {
        let input = "{\"scheme\": \"VISA\", \"type\": \"CARD\", \"account_number\": \"1234123412341234\", \"expiry_year\": 2021, \"expiry_month\": 12, \"security_code\": \"123\"}";
        let actual: PaymentRequest = serde_json::from_str(input).expect("deserialize PaymentRequest");
        let expected = PaymentRequest {
            payment_type: PaymentType::Card { scheme: CardScheme::Visa, expiry_date: (2021, 12), security_code: "123".to_string()},
            account_number: "1234123412341234".to_string()
        };
        assert_eq!(expected, actual);

        let input = "{\"type\": \"ACCOUNT\", \"account_number\": \"1234123412341234\"}";
        let actual: PaymentRequest = serde_json::from_str(input).expect("deserialize PaymentRequest");
        let expected = PaymentRequest {
            payment_type: PaymentType::Account,
            account_number: "1234123412341234".to_string()
        };
        assert_eq!(expected, actual);

        let input = "{\"scheme\": \"VISA\", \"type\": \"ACCOUNT\", \"account_number\": \"1234123412341234\", \"expiry_year\": 2021, \"expiry_month\": 12, \"security_code\": \"123\"}";
        let actual = serde_json::from_str::<PaymentRequest>(input).map_err(|e| e.to_string());
        let expected = Err("unknown field `expiry_month, expiry_year, security_code, scheme`, expected `type` or `account_number` at line 1 column 140".into());
        assert_eq!(expected, actual);
    }
}