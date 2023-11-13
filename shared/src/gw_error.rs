use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum GwError {
    EncodeError,
    DecodeError,
    ConnectionError,
}

impl Error for GwError {}

impl Display for GwError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::EncodeError => "encoding error",
            Self::DecodeError => "decoding error",
            Self::ConnectionError => "connection error",
        };
        write!(f, "{}", msg)
    }
}

impl From<std::io::Error> for GwError {
    fn from(value: std::io::Error) -> Self {
        Self::ConnectionError
    }
}
