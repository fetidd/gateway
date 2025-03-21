#![allow(async_fn_in_trait)]

use crate::{error::Error, pool::Pool};

pub trait Persist
where
    Self: Sized,
{
    type Id: Sized;
    type FindArgs;

    fn table_name(&self) -> &'static str;

    async fn save(&self, pool: &Pool) -> Result<(), Error>;
    async fn load(id: impl Into<Self::Id>) -> Result<Self, Error>;
    async fn reload(&self, pool: &Pool) -> Result<(), Error>;
    async fn find(filter: Self::FindArgs) -> Result<Self, Error>;
}
