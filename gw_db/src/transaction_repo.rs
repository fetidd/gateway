use crate::{
    error::{Error, Result},
    repo::Repo,
};
use gw_core::transaction::Transaction;
use sqlx::{Acquire, Connection, Sqlite};

pub struct TransactionRepo<'a> {
    pool: &'a sqlx::Pool<Sqlite>,
}

pub struct TransactionFilter;

pub struct TransactionUpdater;

impl<'a> Repo for TransactionRepo<'a> {
    type Entity = Transaction;
    type Id = String;
    type Filter = TransactionFilter;
    type Updater = TransactionUpdater;

    async fn create(&self, entity: Self::Entity) -> Result<()> {
        let query_str = format!(
            "INSERT INTO {}_transaction (id, baseamount) VALUES (?,?);",
            entity.acquirer
        );
        sqlx::query(&query_str)
            .bind(entity.id)
            .bind(entity.baseamount)
            .execute(self.pool)
            .await
            .map_err(|e| Error::QueryError(e.to_string()))?;
        Ok(())
    }

    async fn read_one(&self, id: Self::Id) -> Result<Self::Entity> {
        todo!()
    }

    async fn read_many(&self, filter: Option<Self::Filter>) -> Result<Vec<Self::Entity>> {
        todo!()
    }

    async fn delete_one(&self, id: Self::Id) -> Result<bool> {
        todo!()
    }

    async fn delete_many(&self, filter: Option<Self::Filter>) -> Result<bool> {
        todo!()
    }

    async fn update_one(&self, entity: Self::Entity) -> Result<bool> {
        todo!()
    }

    async fn update_many(
        &self,
        filter: Option<Self::Filter>,
        update: Self::Updater,
    ) -> Result<bool> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use sqlx::SqlitePool;

    use super::*;

    #[tokio::test]
    async fn test_create() {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::query("CREATE TABLE IF NOT EXISTS bank_transaction (id TEXT, baseamount INTEGER);")
            .execute(&pool)
            .await
            .unwrap();
        let repo = TransactionRepo { pool: &pool };
        let trx = Transaction {
            id: "todo_uuid".into(),
            baseamount: 20000,
            acquirer: "bank".into(),
        };
        assert!(repo.create(trx).await.is_ok());
    }
}
