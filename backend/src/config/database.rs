use log::{error, info};
use sqlx::{Error as SqlxError, PgPool, postgres::PgPoolOptions};
use std::time::Duration;

use crate::constants::DATABASE_URL;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new() -> Result<Self, SqlxError> {
        info!(
            "Attempting to connect to PostgreSQL at {}",
            DATABASE_URL.as_str()
        );

        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(DATABASE_URL.as_str())
            .await
        {
            Ok(pool) => {
                info!("✅ Connected to PostgreSQL (Supabase)");
                pool
            }
            Err(e) => {
                error!("❌ Failed to connect to PostgreSQL: {}", e);
                return Err(e);
            }
        };

        Ok(Self { pool })
    }
}
