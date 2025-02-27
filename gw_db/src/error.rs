#[derive(Debug)]
pub enum Error {
    ConnectionError,
    QueryError(String),
}

pub type Result<T> = std::result::Result<T, Error>;
