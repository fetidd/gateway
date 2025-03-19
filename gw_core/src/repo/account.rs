use std::sync::Arc;

use crate::{
    account::{AcquirerAccount, BankOneAccount, BankTwoAccount},
    currency::Currency,
    error::{DbErrorKind, Error, ErrorKind},
    payment::Payment,
};

use super::*;

#[derive(Debug)]
pub struct AccountRepo {
    pub pool: Arc<Pool>,
}

impl Repo for AccountRepo {
    type Entity = AcquirerAccount;

    type Id = i32;

    fn pool(&self) -> &PgPool {
        &**self.pool
    }
}

impl Entity for AcquirerAccount {
    fn values_str(&self) -> String {
        match self {
            AcquirerAccount::BankOne(bank_one_account) => todo!(),
            AcquirerAccount::BankTwo(bank_two_account) => todo!(),
        }
    }

    fn bind_to<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments> {
        match self {
            AcquirerAccount::BankOne(bank_one_account) => todo!(),
            AcquirerAccount::BankTwo(bank_two_account) => todo!(),
        }
    }

    fn table_name(&self) -> &'static str {
        match self {
            AcquirerAccount::BankOne(..) => "account.bankone",
            AcquirerAccount::BankTwo(..) => "account.bankone",
        }
    }
}

impl<'r> FromRow<'r, PgRow> for AcquirerAccount {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let table_name: &str = row.get("table_name"); // using 'tableoid::regclass::text as table_name' in the select, this allows us to know which acquirer we are turnting the row into
        match table_name {
            "account.bankone" => Ok(AcquirerAccount::BankOne(BankOneAccount::from_row(row)?)),
            "account.banktwo" => Ok(AcquirerAccount::BankTwo(BankTwoAccount::from_row(row)?)),
            invalid => panic!("{invalid} is invalid acquirer"),
        }
    }
}

impl AccountRepo {
    pub async fn select_for(
        &self,
        merchant_id: &str,
        payment_data: &Payment,
        currency: Currency,
    ) -> Result<AcquirerAccount, Error> {
        let scheme = match *payment_data {
            Payment::Card { scheme, .. } => scheme,
            Payment::Account { .. } => todo!(),
        };
        let res = sqlx::query("SELECT DISTINCT acquirer, account_id FROM account.paymentroute WHERE scheme = $1 and currency = $2 and merchant_id = $3 LIMIT 1;")
            .bind(scheme.to_string())
            .bind(currency.to_string())
            .bind(merchant_id)
            .fetch_one(&**self.pool)
            .await;
        if let Err(error) = res {
            match error {
                sqlx::Error::RowNotFound => {
                    return Err(Error {
                        kind: ErrorKind::Database(DbErrorKind::Query),
                        message: "no account found".into(),
                    });
                }
                other => return Err(other.into()),
            }
        }
        let res = res.unwrap();
        let acquirer: &str = res.get_unchecked("acquirer");
        let account_id: i32 = res.get_unchecked("account_id");
        self.select_one(&account_id, &format!("account.{acquirer}"))
            .await
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
    use super::*;
    use crate::{
        card_scheme::CardScheme,
        error::{DbErrorKind, ErrorKind},
    };

    #[sqlx::test]
    async fn test_select_for(pool: PgPool) {
        let pool = Arc::new(Pool { _pool: pool });
        let repo = AccountRepo { pool };
        let payment_data = Payment::Card {
            scheme: CardScheme::Visa,
            security_code: "123".into(),
            expiry_date: (2025, 12),
            pan: "4111111111111111".into(),
        };
        let actual = repo
            .select_for("merchant123", &payment_data, Currency::GBP)
            .await;
        let expected = AcquirerAccount::BankOne(BankOneAccount {
            merchant_identification_value: "merchant123".into(),
        });
        assert_eq!(actual.unwrap(), expected);
    }

    #[sqlx::test]
    async fn test_select_for_missing_account(pool: PgPool) {
        let pool = Arc::new(Pool { _pool: pool });
        let repo = AccountRepo { pool };
        let payment_data = Payment::Card {
            scheme: CardScheme::Mastercard,
            security_code: "123".into(),
            expiry_date: (2025, 12),
            pan: "5000111122223333".into(),
        };
        let actual = repo
            .select_for("merchant123", &payment_data, Currency::GBP)
            .await;
        let expected_kind = ErrorKind::Database(DbErrorKind::Query);
        let expected_msg = "no account found";
        let err = actual.unwrap_err();
        assert_eq!(err.kind, expected_kind);
        assert_eq!(err.message, expected_msg);
        assert_eq!(err.to_string(), "DatabaseError [Query]: no account found");
    }
}
