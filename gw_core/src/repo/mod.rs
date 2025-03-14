pub mod account;
pub mod merchant;
// pub mod transaction;

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
            .map_err(DatabaseError::from)?;
        Ok(Pool { _pool })
    }
}

impl Deref for Pool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self._pool
    }
}

pub(crate) trait Repo {
    type Entity: Entity;
    type Id;

    #[allow(async_fn_in_trait)] // only using in own code
    async fn select_one(&self, id: &Self::Id) -> Result<Self::Entity, DatabaseError>
    where
        for<'i> <Self as Repo>::Id: std::fmt::Display + Encode<'i, Postgres> + Type<Postgres>,
    {
        let res = query_as::<_, Self::Entity>(&format!(
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
    async fn insert_one<'e>(&self, entity: &'e Self::Entity) -> Result<Self::Id, DatabaseError>
    where
        for<'a> <Self as Repo>::Id: Decode<'a, Postgres> + Type<Postgres>,
    {
        let stmt = format!(
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

    fn table_name(&self) -> &'static str;
    fn pool(&self) -> &PgPool;
}

pub(crate) trait Entity: for <'r> FromRow<'r, PgRow> + Send + Unpin {
    
    /// The string to be passed into the SQL INSERT query after VALUES
    fn values_str(&self) -> String;

    /// Takes in a Query so that the Entity can correctly bind its parameters to it
    /// in the order specified in values_str
    fn bind_to<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{test, Error, Row};

    #[test]
    async fn test_insert_one(pool: PgPool) -> Result<(), DatabaseError> {
        sqlx::query("create table test (id text, name text, primary key(id));")
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
        sqlx::query("create table test (id serial PRIMARY KEY, name text);")
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

    impl<'a> FromRow<'a, PgRow> for TestDomain {
        fn from_row(row: &'a PgRow) -> Result<Self, Error> {
            let id: i32 = row.try_get("id")?;
            let name: String = row.try_get("name")?;
            let res = TestDomain { id , name };
            Ok(res)
        }
    }

    impl Entity for TestDomain {
        fn bind_to<'a>(
            &'a self,
            stmt: Query<'a, Postgres, PgArguments>,
        ) -> Query<'a, Postgres, PgArguments> {
            stmt.bind(self.name.clone())
        }

        fn values_str(&self) -> String {
            "DEFAULT, $1".into()
        }
    }

    impl<'a> FromRow<'a, PgRow> for TestDomainNoAuto {
        fn from_row(row: &'a PgRow) -> Result<Self, Error> {
            let number: i32 = row.try_get("number")?;
            let name: String = row.try_get("name")?;
            let res = TestDomainNoAuto { name, number };
            Ok(res)
        }
    }

    impl Entity for TestDomainNoAuto {
        fn bind_to<'a>(&'a self, stmt: Query<'a, Postgres, PgArguments>) -> Query<'a, Postgres, PgArguments> {
            stmt.bind(self.number).bind(self.name.clone())
        }

        fn values_str(&self) -> String {
            "$1, $2".into()
        }
    }

    impl Repo for TestRepo {
        type Entity = TestDomain;
        type Id = i32;

        fn pool(&self) -> &PgPool {
            &self.pool
        }

        fn table_name(&self) -> &'static str {
            todo!()
        }
    }

    impl Repo for TestRepoNoAuto {
        type Entity = TestDomainNoAuto;
        type Id = String;

        fn pool(&self) -> &PgPool {
            &self.pool
        }

        fn table_name(&self) -> &'static str {
            todo!()
        }
    }
}
