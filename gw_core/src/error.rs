#[derive(Debug)]
pub struct Error {
    pub(crate) kind: ErrorKind,
    pub(crate) message: String,
    // pub(crate) source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::Database(db_err_kind) => {
                write!(f, "DatabaseError [{db_err_kind}]: {}", self.message)
            }
            ErrorKind::Type => write!(f, "TypeError: {}", self.message),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    Database(DbErrorKind),
    Type,
}

#[derive(Debug, PartialEq)]
pub enum DbErrorKind {
    Query,
    Connection,
    Other,
}

impl std::fmt::Display for DbErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbErrorKind::Query => write!(f, "Query"),
            DbErrorKind::Connection => write!(f, "Connection"),
            DbErrorKind::Other => write!(f, "Other"),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Error {
                kind: ErrorKind::Database(DbErrorKind::Query),
                message: "no records returned".into(),
            },
            sqlx::Error::Configuration(..)
            | sqlx::Error::Database(..)
            | sqlx::Error::Io(..)
            | sqlx::Error::Tls(..)
            | sqlx::Error::Protocol(..)
            | sqlx::Error::TypeNotFound { .. }
            | sqlx::Error::ColumnIndexOutOfBounds { .. }
            | sqlx::Error::ColumnNotFound(..)
            | sqlx::Error::ColumnDecode { .. }
            | sqlx::Error::Encode(..)
            | sqlx::Error::Decode(..)
            | sqlx::Error::AnyDriverError(..)
            | sqlx::Error::PoolTimedOut
            | sqlx::Error::PoolClosed
            | sqlx::Error::WorkerCrashed
            | sqlx::Error::Migrate(..)
            | _ => {
                dbg!(value);
                Error {
                    kind: ErrorKind::Database(DbErrorKind::Other),
                    message: "TODO_ERROR: {value:?}".into(),
                }
            }
        }
    }
}
