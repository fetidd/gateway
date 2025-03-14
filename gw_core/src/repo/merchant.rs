use std::str::FromStr;

use crate::{country::Country, error::DatabaseError, merchant::Merchant};
use sqlx::{Encode, PgPool, Postgres};

use super::{Pool, Repo};

#[derive(Debug, Clone)]
pub struct MerchantRepo {
    pub pool: Box<Pool>,
}

impl Repo for MerchantRepo {
    type Entity = Merchant;
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
