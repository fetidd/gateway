use crate::account::Account;

use super::*;

pub struct AccountRepo {
    pool: Box<Pool>
}

pub struct AccountRecord {}

impl Repo for AccountRepo {
    type Entity = Box<dyn Account + Send + Sync>;
    type Id = i32;

    fn pool(&self) -> &PgPool {
        &self.pool
    }

    fn table_name(&self) -> &'static str {
        todo!()
    }

    async fn select_one(&self, id: &Self::Id) -> Result<Self::Entity, DatabaseError>
    where
        for<'i> <Self as Repo>::Id: std::fmt::Display + Encode<'i, Postgres> + Type<Postgres>,
    {
        let res = query_as::<_, Self::Entity>(&std::format!(
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

    async fn insert_one<'e>(&self, entity: &'e Self::Entity) -> Result<Self::Id, DatabaseError>
    where
        for<'a> <Self as Repo>::Id: Decode<'a, Postgres> + Type<Postgres>,
    {
        let stmt = std::format!(
            "INSERT INTO {} VALUES ({}) RETURNING id",
            self.table_name(),
            entity.values_str(),
        );
        let query = sqlx::query(&stmt);
        let query = entity.bind_to(query);
        let res = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| DatabaseError::from(e))?;
        let id: Self::Id = res.get::<Self::Id, &str>("id");
        Ok(id)
    }
}


impl<'r> FromRow<'r, PgRow> for Box<dyn Account + Send + Sync> {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        todo!()
    }
}

impl Entity for Box<dyn Account + Send + Sync> {
    fn values_str(&self) -> String {
        todo!()
    }

    fn bind_to<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments> {
        todo!()
    }
}
