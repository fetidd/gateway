pub trait Account: std::fmt::Debug {
    fn hash(&self) -> u64;
    fn bank_name(&self) -> String;
    fn mid(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct BankOneAccount {
    pub merchant_identification_value: String,
}

impl Account for BankOneAccount {
    fn hash(&self) -> u64 {
        return 1; // TODO obvs this isnt a hash
    }

    fn bank_name(&self) -> String {
        "Bank One".into()
    }

    fn mid(&self) -> String {
        self.merchant_identification_value.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct BankTwoAccount {
    pub merchant_reference: String,
}

impl Account for BankTwoAccount {
    fn hash(&self) -> u64 {
        2
    }

    fn bank_name(&self) -> String {
        "Bank Two".into()
    }

    fn mid(&self) -> String {
        self.merchant_reference.to_owned()
    }
}
