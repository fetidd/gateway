pub mod merchant;

use std::ops::Deref;

use sqlx::{
    postgres::{PgArguments, PgPoolOptions, PgRow},
    prelude::Type,
    query::Query,
    query_as, Encode, FromRow, PgPool, Postgres,
};

use crate::error::DatabaseError;

#[derive(Debug, Clone)]
pub struct Pool {
    _pool: PgPool,
}

impl Pool {
    pub async fn new(path: &str) -> Result<Pool, DatabaseError> {
        let _pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(path)
            .await
            .map_err(|e| DatabaseError::from(e))?;
        Ok(Pool { _pool })
    }
}

impl Deref for Pool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self._pool
    }
}

pub trait Repo {
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
            "INSERT INTO {} VALUES ({})",
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
    use sqlx::{test, Row};

    struct TestRepo {
        pub pool: PgPool,
    }

    impl TestRepo {
        fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    #[derive(PartialEq, Debug)]
    struct TestDomain {
        id: i32,
        name: String,
    }

    #[derive(FromRow)]
    struct TestRecord {
        id: i32,
        name: String,
    }

    impl From<&TestDomain> for TestRecord {
        fn from(value: &TestDomain) -> Self {
            TestRecord {
                id: value.id as i32,
                name: value.name.to_owned(),
            }
        }
    }

    impl TryFrom<TestRecord> for TestDomain {
        type Error = DatabaseError;

        fn try_from(value: TestRecord) -> Result<Self, Self::Error> {
            let TestRecord { id, name } = value;
            Ok(TestDomain { id, name })
        }
    }

    impl DbRecord for TestRecord {
        fn num_args(&self) -> usize {
            2
        }

        fn bind_to(self, stmt: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments> {
            stmt.bind(self.id).bind(self.name)
        }
    }

    impl Repo for TestRepo {
        type Domain = TestDomain;
        type Record = TestRecord;
        type Id = i32;

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
        let repo = TestRepo::new(pool.clone());
        let domain = TestDomain {
            id: 0,
            name: "test".into(),
        }; // we dont want an id here..
        let res = repo.insert_one(&domain).await?;
        dbg!(res);
        let res = sqlx::query("select * from test where id = 0")
            .fetch_one(&pool)
            .await?;
        assert_eq!(res.get::<i32, &str>("id"), 0);
        assert_eq!(res.get::<String, &str>("name"), "test".to_string());
        Ok(())
    }

    #[test]
    fn test_select_one(pool: PgPool) -> Result<(), DatabaseError> {
        sqlx::query("create table test (id integer, name text)")
            .execute(&pool)
            .await?;
        sqlx::query("insert into test values (1, 'test')")
            .execute(&pool)
            .await?;
        let repo = TestRepo::new(pool);
        let res = repo.select_one(&1).await?;
        assert_eq!(
            res,
            TestDomain {
                id: 1,
                name: "test".into()
            }
        );
        Ok(())
    }
}
