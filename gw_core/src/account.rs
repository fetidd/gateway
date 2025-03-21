
use sqlx::{postgres::PgRow, FromRow, Row};

use crate::{
    currency::Currency,
    error::{DbErrorKind, Error, ErrorKind},
    payment::Payment,
    pool::Pool,
};

#[derive(Debug, PartialEq)]
pub enum Account {
    BankOne(BankOneAccount),
    BankTwo(BankTwoAccount),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BankOneAccount {
    pub merchant_identification_value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BankTwoAccount {
    pub merchant_reference: String,
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

pub async fn select_for(
    merchant_id: &str,
    payment_data: &Payment,
    currency: Currency,
    pool: &Pool,
) -> Result<Account, Error> {
    let scheme = match *payment_data {
        Payment::Card { scheme, .. } => scheme,
        Payment::Account { .. } => todo!(),
    };
    let res = sqlx::query("SELECT DISTINCT acquirer, account_id FROM paymentroute WHERE scheme = $1 and currency = $2 and merchant_id = $3 LIMIT 1;")
            .bind(scheme.to_string())
            .bind(currency.to_string())
            .bind(merchant_id)
            .fetch_one(&**pool)
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
    load(account_id, acquirer, pool).await
}

async fn load(id: i32, acquirer: &str, pool: &Pool) -> Result<Account, Error> {
    let s = format!("SELECT * FROM account.{acquirer} WHERE id = $1");
    let q = sqlx::query(&s).bind(id);
    let account = q.fetch_one(&**pool).await?;
    Ok(match acquirer {
        "bankone" => BankOneAccount::from_row(&account)?.into(),
        "banktwo" => BankTwoAccount::from_row(&account)?.into(),
        _ => todo!(),
    })
}

impl From<BankOneAccount> for Account {
    fn from(value: BankOneAccount) -> Self {
        Self::BankOne(value)
    }
}

impl From<BankTwoAccount> for Account {
    fn from(value: BankTwoAccount) -> Self {
        Self::BankTwo(value)
    }
}

// trait Iso8853<'a> {
//     fn merchant_id(&'a self) -> &'a str;
// }

// impl<'a> Iso8853<'a> for BankOneAccount {
//     fn merchant_id(&'a self) -> &'a str {
//         &self.merchant_identification_value
//     }
// }

// trait Apacs30<'a> {
//     fn merchant_id(&'a self) -> &'a str;
// }

// impl<'a> Apacs30<'a> for BankOneAccount {
//     fn merchant_id(&'a self) -> &'a str {
//         &self.merchant_identification_value[0..1]
//     }
// }

#[cfg(test)]
mod tests {
    

    // #[test]
    // fn access_inner() {
    //     let acq_acct = AcquirerAccount::BankOne(BankOneAccount {
    //         merchant_identification_value: "123".into(),
    //     });
    //     if let AcquirerAccount::BankOne(acct) = acq_acct {
    //         assert_eq!(<BankOneAccount as Iso8853>::merchant_id(&acct), "123");
    //         assert_eq!(<BankOneAccount as Apacs30>::merchant_id(&acct), "1");
    //     }
    // }
}
