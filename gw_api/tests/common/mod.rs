use std::sync::Arc;

use axum_test::TestServer;
use gw_api::app::{create_router, AppState};
use gw_core::pool::Pool;
use serde_json::{Map, Value};

pub fn create_server(pool: sqlx::PgPool) -> TestServer {
    let pool = Pool::from(pool);
    let app_state = Arc::new(AppState { pool });
    let router = create_router(app_state);
    let server = TestServer::new(router).expect("creating server failed");
    server
}

#[derive(Clone)]
pub enum CreateRequestAction {
    Modify(Vec<String>, serde_json::Value),
    Delete(Vec<String>),
}

impl<T: Into<serde_json::Value>> From<(&str, T)> for CreateRequestAction {
    fn from(value: (&str, T)) -> Self {
        let path: Vec<String> = value.0.split(".").map(String::from).collect();
        Self::Modify(path, value.1.into())
    }
}

impl From<&str> for CreateRequestAction {
    fn from(value: &str) -> Self {
        assert!(!value.is_empty(), "path is empty!");
        let mut path: Vec<String> = value.split(".").map(String::from).collect();
        assert!(
            path[0].chars().nth(0).is_some_and(|ch| ch == '!'),
            "delete action missing the prefix '!'"
        );
        path[0] = path[0].trim_start_matches('!').to_string();
        Self::Delete(path)
    }
}

/// Creates a VISA CARD AUTH request, whose values can be overriden by passing a slice of overrides of this type: (&str, serde_json::Value).
/// The &str key can be made into a path by separating nodes using '.'. Each section of the path corresponds to a key/value pair in a JSON object.
pub fn create_request<'a>(overrides: Vec<CreateRequestAction>) -> serde_json::Value {
    let default_payment = serde_json::Map::from_iter([
        ("scheme".into(), serde_json::Value::String("VISA".into())),
        (
            "pan".into(),
            serde_json::Value::String("4000111122223333".into()),
        ),
        (
            "security_code".into(),
            serde_json::Value::String("123".into()),
        ),
        ("expiry_year".into(), serde_json::Value::Number(2026.into())),
        ("expiry_month".into(), serde_json::Value::Number(12.into())),
        (
            "payment_type".into(),
            serde_json::Value::String("CARD".into()),
        ),
    ]);
    let default_billing =
        serde_json::Map::from_iter([("country".into(), serde_json::Value::String("GB".into()))]);
    let default = serde_json::Map::from_iter([
        ("amount".into(), serde_json::Value::Number(12345.into())),
        ("currency".into(), serde_json::Value::String("GBP".into())),
        (
            "transaction_type".into(),
            serde_json::Value::String("Auth".into()),
        ),
        (
            "merchant_id".into(),
            serde_json::Value::String("merchant123".into()),
        ),
        ("payment".into(), default_payment.into()),
        ("billing".into(), default_billing.into()),
    ]);
    let mut default: Value = default.into();
    // apply any overrides
    for action in overrides.into_iter() {
        // let action: CreateRequestAction = override_pair.into();
        match action {
            CreateRequestAction::Modify(path, value) => {
                perform_action(
                    &mut default,
                    |map| {
                        map.entry(path.last().unwrap())
                            .and_modify(|x| *x = value.clone());
                    },
                    &path,
                );
            }
            CreateRequestAction::Delete(path) => {
                perform_action(
                    &mut default,
                    |map| {
                        map.remove(path.last().unwrap());
                    },
                    &path,
                );
            }
        };
    }
    default.into()
}

fn perform_action<F: FnMut(&mut Map<String, Value>)>(
    mut obj: &mut Value,
    mut action: F,
    path: &[String],
) {
    for node in path[..path.len() - 1].iter() {
        obj = obj.get_mut(&node).unwrap();
    }
    match obj {
        Value::Object(map) => {
            action(map);
        }
        _ => panic!("end node is not a Map"),
    }
}
