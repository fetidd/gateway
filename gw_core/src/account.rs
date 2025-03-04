pub trait Account: std::fmt::Debug {
    fn hash(&self) -> u64;
}
