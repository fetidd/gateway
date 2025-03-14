use crate::account::Account;

use super::*;

pub struct AccountRepo {}

pub struct AccountRecord {}

impl Repo for AccountRepo {
    type Domain = Box<dyn Account>;

    type Record = AccountRecord;

    type Id = i32;

    fn table_name(&self) -> &str {
        todo!()
    }

    fn pool(&self) -> &PgPool {
        todo!()
    }
}
