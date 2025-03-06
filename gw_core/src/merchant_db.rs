use crate::error::DatabaseError;
use sqlx::{self, postgres::PgPoolOptions, PgPool};

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

    pub async fn select_merchant(&self, id: u32) -> Result<Option<MerchantRecord>, DatabaseError> {
        sqlx::query_as::<_, MerchantRecord>("SELECT * FROM merchant WHERE id = ?")
            .bind(id)
            .fetch(&self.pool)
            .await?
    }
}

#[derive(sqlx::FromRow)]
pub struct MerchantRecord {
    merchant_id: String,
    name: String,
    premise: String,
    street: String,
    city: String,
    postcode: String,
    county: String,
    country: String,
}
