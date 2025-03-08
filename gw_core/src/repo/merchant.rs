use std::str::FromStr;

use crate::{country::Country, error::DatabaseError, merchant::Merchant};
use sqlx::{postgres::PgPoolOptions, PgPool};

use super::Repo;

#[derive(Debug, Clone)]
pub struct MerchantRepo {
    pool: PgPool,
}

impl MerchantRepo {
    pub async fn connect(path: &str) -> Result<MerchantRepo, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(path)
            .await
            .map_err(|e| DatabaseError::from(e))?;
        Ok(MerchantRepo { pool })
    }

    pub async fn select_merchant(&self, id: &str) -> Result<Merchant, DatabaseError> {
        self.select_one(&id.into()).await
    }
}

impl Repo for MerchantRepo {
    type Domain = Merchant;
    type Record = MerchantRecord;
    type Id = String;

    fn table_name(&self) -> &str {
        "account.merchant"
    }

    fn pool(&self) -> &PgPool {
        &self.pool
    }
}

#[derive(sqlx::FromRow, Default)]
pub struct MerchantRecord {
    id: String,
    name: String,
    premise: String,
    street: String,
    city: String,
    postcode: String,
    county: String,
    country: String,
}

impl TryFrom<MerchantRecord> for Merchant {
    type Error = DatabaseError; // TODO give this a proper error
    fn try_from(value: MerchantRecord) -> Result<Self, Self::Error> {
        let MerchantRecord {
            id,
            name,
            premise,
            street,
            city,
            postcode,
            county,
            country,
        } = value;
        let country = Country::from_str(&country)
            .map_err(|e| DatabaseError::QueryError(format!("bad country: {e}")))?;
        Ok(Merchant {
            merchant_id: id,
            name,
            premise,
            street,
            city,
            postcode,
            county,
            country,
        })
    }
}
