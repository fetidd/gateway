use std::sync::Arc;

use sqlx::{
    postgres::{PgArguments, PgRow},
    query::Query,
    FromRow, Postgres, Row,
};

use crate::{account::AcquirerAccount, payment::Payment, transaction::Transaction};

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
        let num_base_args = 5; // reference, amount, currency, transaction_type, result
        let num_payment_args = match self.payment {
            Payment::Card { .. } => 4,
            Payment::Account { .. } => 2,
        };
        let num_billing_args = 7; // name, premise, street, city, postcode, county, country
        let num_customer_args = 7; // name, premise, street, city, postcode, county, country
        let num_account_args = match self.account {
            // TODO get these from the actual accounts?
            AcquirerAccount::BankOne(..) => 1,
            AcquirerAccount::BankTwo(..) => 1,
        };
        let total = num_base_args
            + num_payment_args
            + num_billing_args
            + num_customer_args
            + num_account_args;
        (1..=total)
            .into_iter()
            .map(|n| format!("${n}"))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn bind_to_insert<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments> {
        todo!()
    }

    fn values_str_for_update(&self) -> String {
        let num_base_args = 4; // amount, currency, transaction_type, result
        let num_payment_args = match self.payment {
            Payment::Card { .. } => 4,
            Payment::Account { .. } => 2,
        };
        let num_billing_args = 7;
        let num_customer_args = 7;
        let num_account_args = match self.account {
            AcquirerAccount::BankOne(..) => 1,
            AcquirerAccount::BankTwo(..) => 1,
        };
        let total = num_base_args
            + num_payment_args
            + num_billing_args
            + num_customer_args
            + num_account_args;
        (1..=total)
            .into_iter()
            .map(|n| format!("${n}"))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn bind_to_update<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments> {
        todo!()
    }

    fn table_name(&self) -> &'static str {
        match self.account {
            AcquirerAccount::BankOne(..) => "transaction.bankone",
            AcquirerAccount::BankTwo(..) => "transaction.banktwo",
        }
    }
}
