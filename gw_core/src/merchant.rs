use sqlx::{Postgres, Row};

use validify::Validify;

use crate::{country::Country, error::Error};

#[derive(Clone, Debug, PartialEq, Validify, Default)]
pub struct Merchant {
    pub merchant_id: String,
    #[modify(trim)]
    pub name: String,
    #[modify(trim)]
    pub premise: String,
    #[modify(trim)]
    pub street: String,
    #[modify(trim)]
    pub city: String,
    #[modify(trim)]
    pub postcode: String,
    #[modify(trim)]
    pub county: String,
    pub country: Country,
}

impl Merchant {
    // pub async fn save(&self, pool: &crate::pool::Pool) -> Result<(), crate::error::Error> {
    //     let mut query = QueryBuilder::new("INSERT INTO account.merchant");
    //     query
    //         .push_bind(&self.merchant_id)
    //         .push_bind(&self.name)
    //         .push_bind(&self.premise)
    //         .push_bind(&self.street)
    //         .push_bind(&self.city)
    //         .push_bind(&self.postcode)
    //         .push_bind(&self.county)
    //         .push_bind(self.country.to_string());
    //     dbg!(&query.sql());
    //     query
    //         .build()
    //         .execute(&**pool)
    //         .await
    //         .map(|_| ())
    //         .map_err(Error::from)
    // }

    pub async fn load(id: &str, pool: &crate::pool::Pool) -> Result<Self, crate::error::Error> {
        let query =
            sqlx::query_as::<Postgres, Self>("SELECT * FROM merchant WHERE id = $1").bind(id);
        query.fetch_one(&**pool).await.map_err(Error::from)
    }

    // pub async fn reload(&mut self, pool: &crate::pool::Pool) -> Result<(), crate::error::Error> {
    //     let updates = Merchant::load(&self.merchant_id, pool).await?;
    //     *self = updates;
    //     Ok(())
    // }
}

impl<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> for Merchant {
    // TODO test
    fn from_row(row: &'r sqlx::postgres::PgRow) -> Result<Self, sqlx::Error> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    fn test_trim_fields() {
        let mut m = Merchant {
            merchant_id: "merchant123".into(),
            name: "   name   ".into(),
            premise: "   premise   ".into(),
            street: "   street   ".into(),
            city: "   city   ".into(),
            postcode: "   postcode   ".into(),
            county: "   county   ".into(),
            country: Country::GB,
        };
        let exp = Merchant {
            merchant_id: "merchant123".into(),
            name: "name".into(),
            premise: "premise".into(),
            street: "street".into(),
            city: "city".into(),
            postcode: "postcode".into(),
            county: "county".into(),
            country: Country::GB,
        };
        assert!(m.validify().is_ok());
        assert_eq!(m, exp);
    }
}
