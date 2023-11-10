use crate::models::{Model, User};
use crate::schema::users::dsl::*;
use diesel::connection::SimpleConnection;
use diesel::query_builder::QueryFragment;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::Sqlite;
use diesel::{
    Insertable, OptionalExtension, QueryDsl, QueryableByName, RunQueryDsl, SelectableHelper,
    SqliteConnection, Table,
};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::ptr::null;
use std::{env, fmt};

pub struct DatabaseError {
    description: String,
}

impl DatabaseError {
    pub fn new(description: &str) -> Self {
        DatabaseError {
            description: description.to_string(),
        }
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Health Error: {}", self.description)
    }
}

impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Health Error: {}", self.description)
    }
}

impl Error for DatabaseError {}

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

    pub fn run_migrations(&self) {}

    pub fn health_check(&self) -> Result<(), DatabaseError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => conn
                .batch_execute("SELECT 1")
                .map(|_| ())
                .map_err(|_e| DatabaseError::new("Batch execution error")),
            Err(r2d2_error) => Err(DatabaseError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }

    pub fn user_exists(&self, user_uuid: &String) -> Result<bool, DatabaseError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => {
                let user_result = users.find(user_uuid).first::<User>(&mut conn);

                match user_result {
                    Ok(_) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            Err(r2d2_error) => Err(DatabaseError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }

    pub fn user_list(&self, limit: u64) -> Result<Vec<User>, DatabaseError> {
        let mut limit: i64 = limit as i64;
        if limit == 0 {
            limit = std::i64::MAX;
        }

        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => Ok(users
                .limit(limit)
                .select(User::as_select())
                .load(&mut conn)
                .expect("error listing users")),
            Err(r2d2_error) => Err(DatabaseError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }

    pub fn user_get(&self, user_uuid: &String) -> Result<User, DatabaseError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => Ok(users
                .find(user_uuid)
                .first::<User>(&mut conn)
                .expect("Error finding user")),
            Err(r2d2_error) => Err(DatabaseError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }

    pub fn user_add(&self, user_model: User) -> Result<(), DatabaseError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => {
                match diesel::insert_into(users)
                    .values(user_model)
                    .execute(&mut conn)
                {
                    Ok(_) => Ok(()),
                    Err(error) => Err(DatabaseError::new(
                        format!("Error: {}", error.to_string()).as_str(),
                    )),
                }
            }
            Err(r2d2_error) => Err(DatabaseError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }
}
