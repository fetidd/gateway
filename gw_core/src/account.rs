#[derive(Debug, Clone, PartialEq)]
pub struct BankOneAccount {
    pub merchant_identification_value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BankTwoAccount {
    pub merchant_reference: String,
}

#[derive(Debug, PartialEq)]
pub enum AcquirerAccount {
    BankOne(BankOneAccount),
    BankTwo(BankTwoAccount),
}
