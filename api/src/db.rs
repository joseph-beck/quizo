use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{env, fmt};

pub struct HealthError {
    description: String,
}

impl HealthError {
    pub fn new(description: &str) -> Self {
        HealthError {
            description: description.to_string(),
        }
    }
}

impl Display for HealthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Health Error: {}", self.description)
    }
}

impl Debug for HealthError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Health Error: {}", self.description)
    }
}

impl Error for HealthError {}

#[derive(Clone)]
pub struct Database {
    pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
    pub fn new() -> Self {
        let db_url = env::var("SQLITE_DB").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<SqliteConnection>::new(db_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create database connection pool");
        Database { pool }
    }

    pub fn health_check(&self) -> Result<(), HealthError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => conn
                .batch_execute("SELECT 1")
                .map(|_| ())
                .map_err(|_e| HealthError::new("Batch execution error")),
            Err(r2d2_error) => Err(HealthError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }
}
