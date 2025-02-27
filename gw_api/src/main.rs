use std::collections::HashMap;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{
    de::{VariantAccess, Visitor},
    ser::SerializeStruct,
    Deserialize, Serialize,
};
use tracing::{info, instrument, span, Level};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/transaction", post(handle_transaction));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[instrument]
async fn handle_transaction(
    Json(payload): Json<TransactionRequest>,
) -> (StatusCode, Json<TransactionResponse>) {
    let payment = payload.payment.unwrap();
    info!("hello");
    let res = TransactionResponse {
        baseamount: payload.baseamount,
        result: String::from("success"),
        payment: Some(PaymentResponse {
            payment_type: payment.payment_type,
            account_number: payment.account_number.clone(),
        }),
        ..Default::default()
    };
    (StatusCode::CREATED, Json(res))
}

#[derive(Deserialize, Default, Debug)]
struct TransactionRequest {
    baseamount: u32,
    payment: Option<PaymentRequest>,
    billing: Option<BillingRequest>,
}

#[derive(Debug)]
struct PaymentRequest {
    payment_type: PaymentType,
    account_number: String,
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

#[derive(Deserialize, Default, Debug)]
struct BillingRequest {}

#[derive(Serialize, Default)]
struct TransactionResponse {
    baseamount: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment: Option<PaymentResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    billing: Option<BillingResponse>,
    result: String,
}

#[derive(Clone, Copy, Debug)]
enum PaymentType {
    Card { scheme: CardScheme },
    Account,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum CardScheme {
    #[serde(rename = "VISA")]
    Visa,
    #[serde(rename = "MASTERCARD")]
    Mastercard,
}

impl std::fmt::Display for CardScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CardScheme::Visa => "VISA",
            CardScheme::Mastercard => "MASTERCARD",
        };
        write!(f, "{s}")
    }
}

struct PaymentResponse {
    payment_type: PaymentType,
    account_number: String,
}

impl Serialize for PaymentResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("PaymentResponse", 2)?;
        match &self.payment_type {
            PaymentType::Card { scheme } => {
                state.serialize_field("type", "CARD")?;
                state.serialize_field("scheme", &scheme.to_string())?;
            }
            PaymentType::Account => {
                state.serialize_field("type", "ACCOUNT")?;
            }
        }
        state.serialize_field("account_number", &self.account_number)?;
        state.end()
    }
}

#[derive(Serialize, Default)]
struct BillingResponse {}
