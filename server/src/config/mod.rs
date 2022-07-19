use std::sync::Arc;

use sqlx::{Pool, Postgres};

type ContextDbPool = std::sync::Arc<Pool<Postgres>>;
#[derive(Clone)]
pub struct Context {
    pub db_pool: ContextDbPool,
}

impl Context {
    pub fn new(db_pool: Pool<Postgres>) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
        }
    }
}
