pub mod merchant;

use std::ops::Deref;

use sqlx::{
    postgres::{PgArguments, PgPoolOptions, PgRow},
    prelude::Type,
    query::Query,
    query_as, Decode, Encode, FromRow, PgPool, Postgres, Row,
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

    #[allow(async_fn_in_trait)] // only using in own code
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

    #[allow(async_fn_in_trait)] // only using in own code
    async fn insert_one<'r>(&self, obj: &'r Self::Domain) -> Result<Self::Id, DatabaseError>
    where
        <Self as Repo>::Record: From<&'r <Self as Repo>::Domain> + RepoEntity,
        for<'a> <Self as Repo>::Id: Decode<'a, Postgres> + Type<Postgres>,
    {
        let record: Self::Record = obj.into();
        let stmt = format!(
            "INSERT INTO {} VALUES ({}) RETURNING id",
            self.table_name(),
            record.values_str(),
        );
        let query = sqlx::query(&stmt);
        let query = record.bind_to(query);
        let res = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| DatabaseError::from(e))?;
        let id: Self::Id = res.get::<Self::Id, &str>("id");
        Ok(id)
    }

    fn table_name(&self) -> &str;
    fn pool(&self) -> &PgPool;
}

pub trait RepoEntity {
    fn values_str(&self) -> String;
    fn bind_to(self, stmt: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{test, Row};

    #[test]
    async fn test_insert_one(pool: PgPool) -> Result<(), DatabaseError> {
        sqlx::query!("create table test (id text, name text, primary key(id));")
            .execute(&pool)
            .await?;
        let repo = TestRepoNoAuto::new(pool.clone());
        let domain = TestDomainNoAuto {
            number: 123,
            name: "test".into(),
        };
        let res = repo.insert_one(&domain).await?;
        assert_eq!(res, "123");
        let res = sqlx::query("select * from test where id = $1")
            .bind("123")
            .fetch_one(&pool)
            .await?;
        assert_eq!(res.get::<String, &str>("id"), "123".to_string());
        assert_eq!(res.get::<String, &str>("name"), "test".to_string());
        Ok(())
    }

    #[test]
    async fn test_insert_one_auto_id(pool: PgPool) -> Result<(), DatabaseError> {
        sqlx::query!("create table test (id serial PRIMARY KEY, name text);")
            .execute(&pool)
            .await?;
        let repo = TestRepo::new(pool.clone());
        let domain = TestDomain {
            id: 0,
            name: "test".into(),
        };
        let res = repo.insert_one(&domain).await?;
        let res = sqlx::query("select * from test where id = $1")
            .bind(res)
            .fetch_one(&pool)
            .await?;
        assert_eq!(res.get::<i32, &str>("id"), 1);
        assert_eq!(res.get::<String, &str>("name"), "test".to_string());
        Ok(())
    }

    #[test]
    fn test_select_one(pool: PgPool) -> Result<(), DatabaseError> {
        sqlx::query("create table test (id serial, name text)")
            .execute(&pool)
            .await?;
        sqlx::query("insert into test (name) values ($1)")
            .bind("test")
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

    struct TestRepo {
        pub pool: PgPool,
    }

    impl TestRepo {
        fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    struct TestRepoNoAuto {
        pub pool: PgPool,
    }

    impl TestRepoNoAuto {
        fn new(pool: PgPool) -> Self {
            Self { pool }
        }
    }

    #[derive(PartialEq, Debug)]
    struct TestDomain {
        id: i32,
        name: String,
    }

    #[derive(PartialEq, Debug)]
    struct TestDomainNoAuto {
        number: i32,
        name: String,
    }

    #[derive(FromRow)]
    struct TestRecord {
        id: i32,
        name: String,
    }

    #[derive(FromRow)]
    struct TestRecordNoAuto {
        number: i32,
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

    impl From<&TestDomainNoAuto> for TestRecordNoAuto {
        fn from(value: &TestDomainNoAuto) -> Self {
            TestRecordNoAuto {
                number: value.number as i32,
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

    impl TryFrom<TestRecordNoAuto> for TestDomainNoAuto {
        type Error = DatabaseError;

        fn try_from(value: TestRecordNoAuto) -> Result<Self, Self::Error> {
            let TestRecordNoAuto { number, name } = value;
            Ok(TestDomainNoAuto { number, name })
        }
    }

    impl RepoEntity for TestRecord {
        fn bind_to(self, stmt: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments> {
            stmt.bind(self.name)
        }

        fn values_str(&self) -> String {
            "DEFAULT, $1".into()
        }
    }

    impl RepoEntity for TestRecordNoAuto {
        fn bind_to(self, stmt: Query<Postgres, PgArguments>) -> Query<Postgres, PgArguments> {
            stmt.bind(self.number).bind(self.name)
        }

        fn values_str(&self) -> String {
            "$1, $2".into()
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

    impl Repo for TestRepoNoAuto {
        type Domain = TestDomainNoAuto;
        type Record = TestRecordNoAuto;
        type Id = String;

        fn table_name(&self) -> &str {
            "test"
        }

        fn pool(&self) -> &PgPool {
            &self.pool
        }
    }
}
