use shared::codec::{BankCodec, Codec};
use shared::gw_error::GwError;
use shared::transaction::Transaction;
use std::io::Write;
use std::net;

fn main() -> Result<(), shared::gw_error::GwError> {
    let t = Transaction {
        pan: "4123123412341234".into(),
        expiry_date: "12/30".into(),
        amount: 12345,
        currency: "GBP".into(),
        merchant: "Test Merchant".into(),
        billing_name: "Ben Jones".into(),
        status: None,
    };
    let codec = BankCodec::init();
    let mut sender =
        net::TcpStream::connect("localhost:1234").map_err(|e| GwError::ConnectionError)?;
    let msg = codec.encode(&t)?;
    sender
        .write_all(msg.as_bytes())
        .map_err(|e| GwError::ConnectionError)?;
    Ok(())
}
