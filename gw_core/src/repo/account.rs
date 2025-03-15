use std::sync::Arc;

use crate::{
    account::{AcquirerAccount, BankOneAccount, BankTwoAccount},
    currency::Currency,
    payment::Payment,
};

use super::*;

#[derive(Debug)]
pub struct AccountRepo {
    pub pool: Arc<Pool>,
}

impl AccountRepo {
    pub async fn select_for(
        &self,
        merchant_id: &str,
        payment_data: &Payment,
        currency: Currency,
    ) -> Result<AcquirerAccount, DatabaseError> {
        let scheme = match *payment_data {
            Payment::Card { scheme, .. } => scheme,
            Payment::Account { .. } => todo!(),
        };
        let row = sqlx::query("SELECT DISTINCT acquirer, account_id FROM account.paymentroute WHERE scheme = $1 currency = $2, merchant_id = $3 LIMIT 1;")
            .bind(scheme.to_string())
            .bind(currency.to_string())
            .bind(merchant_id)
            .fetch_one(&**self.pool)
            .await?;
        let (acquirer, account_id): (&str, i32) = (
            row.get_unchecked("acquirer"),
            row.get_unchecked("account_id"),
        );
        let sql = format!("SELECT * FROM account.{acquirer} WHERE id = $1");
        let row = sqlx::query(&sql)
            .bind(account_id)
            .fetch_one(&**self.pool)
            .await?;
        make_account(acquirer, &row)
    }
}

fn make_account(acquirer: &str, row: &PgRow) -> Result<AcquirerAccount, DatabaseError> {
    match acquirer {
        "bankone" => Ok(AcquirerAccount::BankOne(BankOneAccount::from_row(&row)?)),
        "banktwo" => Ok(AcquirerAccount::BankTwo(BankTwoAccount::from_row(&row)?)),
        invalid => Err(DatabaseError::QueryError(format!(
            "{invalid} is not a supported acquirer"
        ))),
    }
}

impl<'r> FromRow<'r, PgRow> for BankOneAccount {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(BankOneAccount {
            merchant_identification_value: row.try_get("merchant_identification_value")?,
        })
    }
}

impl<'r> FromRow<'r, PgRow> for BankTwoAccount {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(BankTwoAccount {
            merchant_reference: row.try_get("merchant_reference")?,
        })
    }
}

// impl Entity for BankOneAccount {
//     fn values_str(&self) -> String {
//         "merchant_identification_value".into()
//     }

//     fn bind_to<'a>(
//         &'a self,
//         stmt: Query<'a, Postgres, PgArguments>,
//     ) -> Query<'a, Postgres, PgArguments> {
//         stmt.bind(self.merchant_identification_value.clone())
//     }
// }

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    fn test_make_account() {}
}
