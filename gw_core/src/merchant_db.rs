use std::str::FromStr;

use crate::{country::Country, error::DatabaseError, merchant::Merchant};
use sqlx::{self, postgres::PgPoolOptions, PgPool};

#[derive(Debug, Clone)]
pub struct MerchantDb {
    pool: PgPool,
}

impl MerchantDb {
    pub async fn connect(path: &str) -> Result<MerchantDb, DatabaseError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(path)
            .await
            .map_err(|e| DatabaseError::from(e))?;
        Ok(MerchantDb { pool })
    }

    pub async fn select_merchant(&self, id: &str) -> Result<Merchant, DatabaseError> {
        let res = sqlx::query_as::<_, MerchantRecord>("SELECT * FROM merchant WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DatabaseError::from(dbg!(e)))?;
        Ok(res.try_into()?)
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
        let MerchantRecord { id, name, premise, street, city, postcode, county, country } = value;
        let country = Country::from_str(&country).map_err(|e| DatabaseError::QueryError(format!("bad country: {e}")))?;
        Ok(Merchant { merchant_id: id, name, premise, street, city, postcode, county, country })
    }
}
