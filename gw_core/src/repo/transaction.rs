use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, Row};

use crate::transaction::Transaction;

use super::{Entity, Pool, Repo};

#[derive(Debug)]
pub struct TransactionRepo {
    pub pool: Arc<Pool>,
}

impl Repo for TransactionRepo {
    type Entity = Transaction;

    type Id = String;

    fn pool(&self) -> &sqlx::PgPool {
        &**self.pool
    }
}

impl<'r> FromRow<'r, PgRow> for Transaction {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let table_name: &str = row.get("table_name");
        todo!()
    }
}

impl Entity for Transaction {
    fn values_str_for_insert(&self) -> String {
        todo!()
    }

    fn bind_to_insert<'a>(
        &'a self,
        stmt: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }

    fn values_str_for_update(&self) -> String {
        todo!()
    }

    fn bind_to_update<'a>(
        &'a self,
        stmt: sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments>,
    ) -> sqlx::query::Query<'a, sqlx::Postgres, sqlx::postgres::PgArguments> {
        todo!()
    }

    fn table_name(&self) -> &'static str {
        match self.account {
            crate::account::AcquirerAccount::BankOne(..) => "transaction.bankone",
            crate::account::AcquirerAccount::BankTwo(..) => "transaction.banktwo",
        }
    }
}
