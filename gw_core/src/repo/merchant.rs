use std::{rc::Rc, sync::Arc};

use crate::merchant::Merchant;
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
            merchant_id: row.try_get("merchant_id")?,
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
    fn values_str(&self) -> String {
        "merchant_id, name, premise, street, city, postcode, country, country".into()
    }

    fn bind_to<'a>(
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
}

impl Repo for MerchantRepo {
    type Entity = Merchant;
    type Id = String;

    fn pool(&self) -> &PgPool {
        &self.pool
    }

    fn table_name(&self) -> &'static str {
        "account.merchant"
    }
}
