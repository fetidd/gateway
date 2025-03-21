use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Pool {
    _pool: PgPool,
}

impl Pool {
    pub async fn new(path: &str) -> Result<Pool, Error> {
        let _pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(path)
            .await
            .map_err(Error::from)?;
        Ok(Pool { _pool })
    }
}

impl Deref for Pool {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self._pool
    }
}

impl From<PgPool> for Pool {
    fn from(value: PgPool) -> Self {
        Self { _pool: value }
    }
}
