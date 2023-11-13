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
        // 0000000010008260000111122223333   Merchant Name       Billing Name        0120
        enc.push_str(&self.get_amount(&transaction));
        enc.push_str(&self.get_currency(&transaction)?);
        enc.push_str(&self.get_pan(&transaction));
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

    fn get_amount(&self, t: &Transaction) -> String {
        format!("{:0>12}", t.amount)
    }

    fn get_currency(&self, t: &Transaction) -> Result<String, GwError> {
        match t.currency.as_str() {
            "GBP" => Ok("826".into()),
            _ => Err(GwError::EncodeError),
        }
    }

    fn get_pan(&self, t: &Transaction) -> String {
        format!("{:<19}", t.pan)
    }
}
