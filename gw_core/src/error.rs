#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Configuration(error) => DatabaseError::ConnectionError(error.to_string()), // TODO format with prefixes base don error type
            sqlx::Error::Database(error) => DatabaseError::ConnectionError(error.to_string()),
            sqlx::Error::Io(error) => DatabaseError::ConnectionError(error.to_string()),
            sqlx::Error::Tls(error) => DatabaseError::ConnectionError(error.to_string()),
            sqlx::Error::Protocol(error) => DatabaseError::ConnectionError(error.to_string()),
            sqlx::Error::AnyDriverError(error) => DatabaseError::ConnectionError(error.to_string()),
            sqlx::Error::Migrate(error) => DatabaseError::ConnectionError(error.to_string()),
            sqlx::Error::PoolTimedOut => DatabaseError::ConnectionError("Pool timed out".into()),
            sqlx::Error::PoolClosed => DatabaseError::ConnectionError("Pool closed".into()),
            sqlx::Error::WorkerCrashed => DatabaseError::ConnectionError("Worker crashed".into()),

            sqlx::Error::RowNotFound => DatabaseError::QueryError("Row not found".to_string()),
            sqlx::Error::TypeNotFound { type_name } => DatabaseError::QueryError(type_name),
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => {
                DatabaseError::QueryError(format!("index={index}, len={len} out of bounds"))
            }
            sqlx::Error::ColumnNotFound(error) => {
                DatabaseError::QueryError(format!("Column not found: {error}"))
            }
            sqlx::Error::ColumnDecode { index: _, source } => {
                DatabaseError::QueryError(source.to_string())
            }
            sqlx::Error::Encode(error) => DatabaseError::QueryError(error.to_string()),
            sqlx::Error::Decode(error) => DatabaseError::QueryError(error.to_string()),
            _ => todo!(),
        }
    }
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            DatabaseError::ConnectionError(e) => format!("ConnectionError: {e}"),
            DatabaseError::QueryError(e) => format!("QueryError: {e}"),
        };
        write!(f, "{msg}")
    }
}
