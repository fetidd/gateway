use sqlx::{postgres::PgRow, prelude::Type, query_as, Encode, FromRow, PgPool, Postgres};

use crate::error::DatabaseError;

pub mod merchant;

trait Repo {
    type Domain;
    type Record;
    type Id;

    async fn select_one(&self, id: &Self::Id) -> Result<Self::Domain, DatabaseError>
    where
        for<'r> <Self as Repo>::Record: FromRow<'r, PgRow> + Send + Unpin,
        for<'i> <Self as Repo>::Id: std::fmt::Display + Encode<'i, Postgres> + Type<Postgres>,
        <Self as Repo>::Domain: TryFrom<<Self as Repo>::Record>,
        <<Self as Repo>::Domain as TryFrom<<Self as Repo>::Record>>::Error: std::fmt::Display,
    {
        let res = query_as::<_, Self::Record>(&format!(
            "SELECT * FROM {} WHERE id = $1",
            self.table_name()
        ))
        .bind(id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| DatabaseError::from(e))?;
        match res.try_into() {
            Ok(r) => Ok(r),
            Err(e) => Err(DatabaseError::QueryError(e.to_string())),
        }
    }

    fn table_name(&self) -> &str;
    fn pool(&self) -> &PgPool;
}
