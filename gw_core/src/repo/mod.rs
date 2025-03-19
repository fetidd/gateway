pub mod account;
pub mod merchant;
pub mod transaction;

use std::ops::Deref;

use sqlx::{
    postgres::{PgArguments, PgPoolOptions, PgRow},
    prelude::Type,
    query::Query,
    query_as, Decode, Encode, FromRow, PgPool, Postgres, Row,
};

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Pool {
    _pool: PgPool,
}

impl Pool {
    pub async fn new(path: &str) -> Result<Pool, Error> {
        let _pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(path)
            .await
            .map_err(Error::from)?;
        Ok(Pool { _pool })
    }
}

impl Deref for Pool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self._pool
    }
}

impl From<PgPool> for Pool {
    fn from(value: PgPool) -> Self {
        Self { _pool: value }
    }
}

pub trait Repo {
    type Entity: Entity;
    type Id;

    #[allow(async_fn_in_trait)] // only using in own code
    async fn select_one(&self, id: &Self::Id, table: &str) -> Result<Self::Entity, Error>
    where
        for<'i> <Self as Repo>::Id: std::fmt::Display + Encode<'i, Postgres> + Type<Postgres>,
    {
        let res = query_as::<_, Self::Entity>(&format!(
            "SELECT *, tableoid::regclass::text as table_name FROM {table} WHERE id = $1",
        ))
        .bind(id)
        .fetch_one(self.pool())
        .await
        .map_err(|e| Error::from(e))?;
        Ok(res)
    }

    #[allow(async_fn_in_trait)] // only using in own code
    async fn insert_one<'e>(&self, entity: &'e Self::Entity) -> Result<Self::Id, Error>
    where
        for<'a> <Self as Repo>::Id: Decode<'a, Postgres> + Type<Postgres>,
    {
        let table_name = entity.table_name();
        let values = entity.values_str_for_insert();
        let stmt = format!("INSERT INTO {table_name} VALUES ({values}) RETURNING id",);
        let query = sqlx::query(&stmt);
        let query = entity.bind_to_insert(query);
        let res = query
            .fetch_one(self.pool())
            .await
            .map_err(|e| Error::from(e))?;
        let id: Self::Id = res.get::<Self::Id, &str>("id");
        Ok(id)
    }

    /// Updates a whole entity in the db with the one passed in
    #[allow(async_fn_in_trait)]
    async fn update_one<'e>(&self, id: &Self::Id, entity: &'e Self::Entity) -> Result<(), Error>
    where
        for<'a> <Self as Repo>::Id: Decode<'a, Postgres> + Type<Postgres>,
        for<'i> <Self as Repo>::Id: std::fmt::Display + Encode<'i, Postgres> + Type<Postgres>,
    {
        let table_name = entity.table_name();
        let values = entity.values_str_for_update();
        let stmt = format!("UPDATE {table_name} SET {values} WHERE id = $1",);
        let query = sqlx::query(&stmt).bind(id);
        let query = entity.bind_to_update(query);
        query
            .execute(self.pool())
            .await
            .map_err(|e| Error::from(e))?;
        Ok(())
    }

    fn pool(&self) -> &PgPool;
}

pub trait Entity: for<'r> FromRow<'r, PgRow> + Send + Unpin {
    /// The string to be passed into the SQL INSERT query after VALUES
    fn values_str_for_insert(&self) -> String;

    /// The string to use after SET in the sql statement
    fn values_str_for_update(&self) -> String;

    /// Takes in a Query so that the Entity can correctly bind its parameters to it
    /// in the order specified in values_str
    fn bind_to_insert<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments>;

    /// Takes in a Query so that the Entity can correctly bind its parameters to it
    /// in the order specified in values_str
    fn bind_to_update<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments>;

    fn table_name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{test, Error, Row};

    #[test]
    async fn test_insert_one(pool: PgPool) {
        sqlx::query("create table test (id text, name text, primary key(id));")
            .execute(&pool)
            .await
            .unwrap();
        let repo = TestRepoNoAuto::new(pool.clone());
        let domain = TestDomainNoAuto {
            number: 123,
            name: "test".into(),
        };
        let res = repo.insert_one(&domain).await.unwrap();
        assert_eq!(res, "123");
        let res = sqlx::query("select * from test where id = $1")
            .bind("123")
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(res.get::<String, &str>("id"), "123".to_string());
        assert_eq!(res.get::<String, &str>("name"), "test".to_string());
    }

    #[test]
    async fn test_insert_one_auto_id(pool: PgPool) {
        sqlx::query("create table test (id serial PRIMARY KEY, name text);")
            .execute(&pool)
            .await
            .unwrap();
        let repo = TestRepo::new(pool.clone());
        let domain = TestDomain {
            id: 0,
            name: "test".into(),
        };
        let res = repo.insert_one(&domain).await.unwrap();
        let res = sqlx::query("select * from test where id = $1")
            .bind(res)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(res.get::<i32, &str>("id"), 1);
        assert_eq!(res.get::<String, &str>("name"), "test".to_string());
    }

    #[test]
    fn test_select_one(pool: PgPool) {
        sqlx::query("create table test (id serial, name text)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("insert into test (name) values ($1)")
            .bind("test")
            .execute(&pool)
            .await
            .unwrap();
        let repo = TestRepo::new(pool);
        let res = repo.select_one(&1, "test").await.unwrap();
        assert_eq!(
            res,
            TestDomain {
                id: 1,
                name: "test".into()
            }
        );
    }

    #[test]
    fn test_update_one(pool: PgPool) {
        sqlx::query("create table test (id serial, name text)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("insert into test (name) values ($1)")
            .bind("test")
            .execute(&pool)
            .await
            .unwrap();
        let repo = TestRepo::new(pool.clone());
        let update = TestDomain {
            id: 2,
            name: "new_name".into(),
        };
        assert!(repo.update_one(&1, &update).await.is_ok());
        let check = sqlx::query("SELECT * FROM test WHERE id = 1")
            .fetch_one(&pool)
            .await
            .unwrap();
        let id: i32 = check.get_unchecked("id");
        let name: &str = check.get_unchecked("name");
        assert_eq!(id, 1);
        assert_eq!(name, "new_name");
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
            let res = TestDomain { id, name };
            Ok(res)
        }
    }

    impl Entity for TestDomain {
        fn bind_to_insert<'a>(
            &'a self,
            stmt: Query<'a, Postgres, PgArguments>,
        ) -> Query<'a, Postgres, PgArguments> {
            stmt.bind(self.name.clone())
        }

        fn values_str_for_insert(&self) -> String {
            "DEFAULT, $1".into()
        }

        fn table_name(&self) -> &'static str {
            "test"
        }

        fn values_str_for_update(&self) -> String {
            "name = $1".into()
        }

        fn bind_to_update<'a>(
            &'a self,
            stmt: Query<'a, Postgres, PgArguments>,
        ) -> Query<'a, Postgres, PgArguments> {
            todo!()
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
        fn bind_to_insert<'a>(
            &'a self,
            stmt: Query<'a, Postgres, PgArguments>,
        ) -> Query<'a, Postgres, PgArguments> {
            stmt.bind(self.number).bind(self.name.clone())
        }

        fn values_str_for_insert(&self) -> String {
            "$1, $2".into()
        }

        fn table_name(&self) -> &'static str {
            "test"
        }

        fn values_str_for_update(&self) -> String {
            todo!()
        }

        fn bind_to_update<'a>(
            &'a self,
            stmt: Query<'a, Postgres, PgArguments>,
        ) -> Query<'a, Postgres, PgArguments> {
            todo!()
        }
    }

    impl Repo for TestRepo {
        type Entity = TestDomain;
        type Id = i32;

        fn pool(&self) -> &PgPool {
            &self.pool
        }
    }

    impl Repo for TestRepoNoAuto {
        type Entity = TestDomainNoAuto;
        type Id = String;

        fn pool(&self) -> &PgPool {
            &self.pool
        }
    }
}
