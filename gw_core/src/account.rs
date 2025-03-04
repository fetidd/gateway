pub trait Account: std::fmt::Debug {
    fn hash(&self) -> u64;
}

#[derive(Debug, Clone)]
pub struct BankOneAccount;

impl Account for BankOneAccount {
    fn hash(&self) -> u64 {
        return 1; // TODO obvs this isnt a hash 
    }
}
