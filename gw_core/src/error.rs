#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError,
    QueryError(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::Configuration(error) => todo!(),
            sqlx::Error::Database(database_error) => todo!(),
            sqlx::Error::Io(error) => todo!(),
            sqlx::Error::Tls(error) => todo!(),
            sqlx::Error::Protocol(_) => todo!(),
            sqlx::Error::RowNotFound => todo!(),
            sqlx::Error::TypeNotFound { type_name } => todo!(),
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => todo!(),
            sqlx::Error::ColumnNotFound(_) => todo!(),
            sqlx::Error::ColumnDecode { index, source } => todo!(),
            sqlx::Error::Encode(error) => todo!(),
            sqlx::Error::Decode(error) => todo!(),
            sqlx::Error::AnyDriverError(error) => todo!(),
            sqlx::Error::PoolTimedOut => todo!(),
            sqlx::Error::PoolClosed => todo!(),
            sqlx::Error::WorkerCrashed => todo!(),
            sqlx::Error::Migrate(migrate_error) => todo!(),
            _ => todo!(),
        }
    }
}
