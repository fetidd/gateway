use std::collections::HashMap;

use gw_core::{card_scheme::CardScheme, payment_type::PaymentType};
use serde::{de::Visitor, Deserialize};
use tracing::{info, span, Level};

#[derive(Debug)]
pub struct PaymentRequest {
    pub payment_type: PaymentType,
    pub account_number: String,
}

impl<'de> Deserialize<'de> for PaymentRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let span = span!(Level::INFO, "deserializing");
        let _guard = span.enter();
        let hm = deserializer.deserialize_map(PaymentRequestVisitor)?; // TODO hsshmap allocates on heap - use an array somehow? we will knowntge max num fields we're gonna take from the json obj
        info!(?hm);
        if let Some(p_type) = hm.get("type") {
            match *p_type {
                "CARD" => match hm.get("scheme") {
                    Some(s) => Ok(Self {
                        payment_type: PaymentType::Card {
                            scheme: match *s {
                                "VISA" => CardScheme::Visa,
                                "MASTERCARD" => CardScheme::Mastercard,
                                _ => todo!(),
                            },
                        },
                        account_number: hm.get("account_number").unwrap().to_string(),
                    }),
                    None => todo!(),
                },
                "ACCOUNT" => todo!(),
                _ => todo!(),
            }
        } else {
            todo!()
        }
    }
}

struct PaymentRequestVisitor;
impl<'v> Visitor<'v> for PaymentRequestVisitor {
    type Value = HashMap<&'v str, &'v str>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "type of CARD, ACCOUNT; CARD requires scheme")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'v>,
    {
        let mut hm: HashMap<&str, &str> = HashMap::new();
        while let Ok(Some(entry)) = map.next_entry() {
            hm.insert(entry.0, entry.1);
        }
        Ok(hm)
    }
}