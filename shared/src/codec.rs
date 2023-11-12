use crate::gw_error::GwError;
use crate::transaction::Transaction;

pub trait Codec {
    fn encode(&self, transaction: &Transaction) -> Result<String, GwError>;
    fn decode(&self, transaction: &mut Transaction, received: &str) -> Result<(), GwError>;
}

pub struct BankCodec {}

impl Codec for BankCodec {
    fn encode(&self, transaction: &Transaction) -> Result<String, GwError> {
        let mut enc = String::from("AUTH MESSAGE:");
        enc.push_str(&format!("{:?}", transaction));
        Ok(enc)
    }

    fn decode(&self, transaction: &mut Transaction, received: &str) -> Result<(), GwError> {
        Ok(())
    }
}

impl BankCodec {
    pub fn init() -> Self {
        BankCodec {}
    }
}
