use std::str::FromStr;

use crate::{country::Country, error::DatabaseError, merchant::Merchant};
use sqlx::{Encode, PgPool, Postgres};

use super::{Pool, Repo};

#[derive(Debug, Clone)]
pub struct MerchantRepo {
    pub pool: Box<Pool>,
}

impl<'pool> Repo for MerchantRepo {
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

impl<'q> Encode<'q, Postgres> for Merchant {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        todo!()
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
