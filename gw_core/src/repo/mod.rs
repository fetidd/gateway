use sqlx::{
    postgres::{PgArguments, PgRow},
    prelude::Type,
    query::{self, Query},
    query_as, Arguments, Encode, FromRow, PgPool, Postgres,
};

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

    async fn insert_one<'r>(&self, obj: &'r Self::Domain) -> Result<(), DatabaseError>
    where
        <Self as Repo>::Record: From<&'r <Self as Repo>::Domain> + DbRecord,
    {
        let record: Self::Record = obj.into();
        let stmt = format!(
            "INSERT INTO {} VALUES (DEFAULT, {})",
            self.table_name(),
            generate_param_str(record.num_args())
        );
        let query = sqlx::query(&stmt);
        let query = record.bind_to(query);
        query
            .execute(self.pool())
            .await
            .map_err(|e| DatabaseError::from(e))?;
        Ok(())
    }

    fn table_name(&self) -> &str;
    fn pool(&self) -> &PgPool;
}

trait DbRecord {
    fn num_args(&self) -> usize;
    fn bind_to(self, stmt: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments>;
}

fn generate_param_str(n: usize) -> String {
    (1..=n)
        .into_iter()
        .map(|n| format!("${n}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::test;

    struct TestRepo {
        pub pool: PgPool,
    }

    impl TestRepo {
        fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    struct TestDomain {
        id: u32,
        name: String,
    }
    struct TestRecord {
        id: u32,
        name: String,
    }

    impl From<&TestDomain> for TestRecord {
        fn from(value: &TestDomain) -> Self {
            TestRecord {
                id: value.id,
                name: value.name.to_owned(),
            }
        }
    }

    impl DbRecord for TestRecord {
        fn num_args(&self) -> usize {
            1
        }

        fn bind_to(self, stmt: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments> {
            stmt.bind(self.name)
        }
    }

    impl Repo for TestRepo {
        type Domain = TestDomain;
        type Record = TestRecord;
        type Id = u32;

        fn table_name(&self) -> &str {
            "test"
        }

        fn pool(&self) -> &PgPool {
            &self.pool
        }
    }

    #[test]
    async fn test_insert_one(pool: PgPool) -> Result<(), DatabaseError> {
        sqlx::query!("create table test (id integer, name text);")
            .execute(&pool)
            .await?;
        let repo = TestRepo::new(pool);
        let domain = TestDomain {
            id: 0,
            name: "test".into(),
        }; // we dont want an id here..
        let res = repo.insert_one(&domain).await?;
        dbg!(res);
        Ok(())
    }
}
