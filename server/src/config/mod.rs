use std::sync::Arc;

use sqlx::{MySql, Pool};

type ContextDbPool = std::sync::Arc<Pool<MySql>>;
#[derive(Clone)]
pub struct Context {
    pub db_pool: ContextDbPool,
}

impl Context {
    pub fn new(db_pool: Pool<MySql>) -> Self {
        Self {
            db_pool: Arc::new(db_pool),
        }
    }
}
