#[derive(Default, Debug)]
pub struct Transaction {
    pub amount: u64,
    pub currency: String,
    pub pan: String,
    pub merchant: String,
    pub billing_name: String,
    pub expiry_date: String,
    pub status: Option<String>,
}
