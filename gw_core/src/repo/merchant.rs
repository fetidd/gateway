use std::sync::Arc;

use crate::{error::Error, merchant::Merchant};
use sqlx::{
    postgres::{PgArguments, PgRow},
    prelude::FromRow,
    query::Query,
    PgPool, Postgres, Row,
};

use super::{Entity, Pool, Repo};

#[derive(Debug, Clone)]
pub struct MerchantRepo {
    pub pool: Arc<Pool>,
}

impl MerchantRepo {
    pub async fn find(&self, id: &str) -> Result<Merchant, Error> {
        self.select_one(&id.into(), "account.merchant").await
    }
}

impl<'r> FromRow<'r, PgRow> for Merchant {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let country = row
            .try_get::<String, &str>("country")?
            .try_into()
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "country".into(),
                source: Box::new(e),
            })?;
        Ok(Merchant {
            merchant_id: row.try_get("id")?,
            name: row.try_get("name")?,
            premise: row.try_get("premise")?,
            street: row.try_get("street")?,
            city: row.try_get("city")?,
            postcode: row.try_get("postcode")?,
            county: row.try_get("county")?,
            country,
        })
    }
}

impl Entity for Merchant {
    fn values_str_for_insert(&self) -> String {
        "id, name, premise, street, city, postcode, country, country".into()
    }

    fn bind_to_insert<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments> {
        stmt.bind(self.merchant_id.clone())
            .bind(self.name.clone())
            .bind(self.premise.clone())
            .bind(self.street.clone())
            .bind(self.city.clone())
            .bind(self.postcode.clone())
            .bind(self.county.clone())
            .bind(self.country.to_string())
    }

    fn table_name(&self) -> &'static str {
        "account.merchant"
    }

    fn values_str_for_update(&self) -> String {
        todo!()
    }

    fn bind_to_update<'a>(
        &'a self,
        stmt: Query<'a, Postgres, PgArguments>,
    ) -> Query<'a, Postgres, PgArguments> {
        todo!()
    }
}

impl Repo for MerchantRepo {
    type Entity = Merchant;
    type Id = String;

    fn pool(&self) -> &PgPool {
        &self.pool
    }
}
