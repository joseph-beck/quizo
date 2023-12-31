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

    pub fn user_update(&self, user_model: User) -> Result<(), DatabaseError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => {
                match diesel::update(users.find(&user_model.uuid))
                    .set(&user_model)
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

    pub fn user_delete(&self, user_uuid: &String) -> Result<(), DatabaseError> {
        let conn_result = self.pool.get();
        match conn_result {
            Ok(mut conn) => match diesel::delete(users.find(user_uuid)).execute(&mut conn) {
                Ok(_) => Ok(()),
                Err(error) => Err(DatabaseError::new(
                    format!("Error: {}", error.to_string()).as_str(),
                )),
            },
            Err(r2d2_error) => Err(DatabaseError::new(
                format!("r2d2 Error: {}", r2d2_error.to_string()).as_str(),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Database;
    use crate::models::User;

    #[test]
    fn test_conn() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let conn_result = db.pool.get();
        match conn_result {
            Ok(_) => assert!(true),
            Err(error) => assert!(false, "Database Connection Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_health_check() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let health_result = db.health_check();
        match health_result {
            Ok(_) => assert!(true),
            Err(error) => assert!(false, "Database Health Check Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_user_exist() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let user_uuid = "0";
        let user_result = db.user_exists(&user_uuid.to_string());
        match user_result {
            Ok(exists) => assert!(exists == false),
            Err(error) => assert!(false, "Database User List Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_user_list() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let user_result = db.user_list(0);
        match user_result {
            Ok(users) => assert!(users.len() > 0),
            Err(error) => assert!(false, "Database User List Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_user_get() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let user = User {
            uuid: "123".to_string(),
            username: "Dave".to_string(),
            password: "1ajwd102390boimada!!4as".to_string(),
            email: "dave@email.com".to_string(),
        };
        db.user_add(user.clone()).unwrap();
        let get_result = db.user_get(&user.uuid);
        match get_result {
            Ok(_) => {
                assert!(db.user_exists(&user.uuid).unwrap());
                db.user_delete(&user.uuid).unwrap();
                assert!(!db.user_exists(&user.uuid).unwrap())
            }
            Err(error) => assert!(false, "Database User Get Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_user_add() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let user = User {
            uuid: "456".to_string(),
            username: "Bob".to_string(),
            password: "!4s789sdfgj232?!7761asd".to_string(),
            email: "bobby@email.com".to_string(),
        };
        let add_result = db.user_add(user.clone());
        match add_result {
            Ok(_) => {
                assert!(db.user_exists(&user.uuid).unwrap());
                db.user_delete(&user.uuid).unwrap();
                assert!(!db.user_exists(&user.uuid).unwrap())
            }
            Err(error) => assert!(false, "Database User Add Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_user_update() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let mut user = User {
            uuid: "789".to_string(),
            username: "Derek".to_string(),
            password: "6782%&123asdcvb!!jzxcqtry56".to_string(),
            email: "derek@email.com".to_string(),
        };
        db.user_add(user.clone()).unwrap();
        assert!(db.user_exists(&user.uuid).unwrap());

        user.username = "Derek123".to_string();
        let update_result = db.user_update(user.clone());
        match update_result {
            Ok(_) => {
                assert!(db.user_get(&user.uuid).unwrap().username.eq(&user.username));
                db.user_delete(&user.uuid).unwrap();
                assert!(!db.user_exists(&user.uuid).unwrap());
            }
            Err(error) => assert!(false, "Database User Update Error: {:?}", error),
        }

        drop(db);
    }

    #[test]
    fn test_user_delete() {
        dotenv::dotenv().ok();

        let db = Database::new();
        let user = User {
            uuid: "012".to_string(),
            username: "Derek".to_string(),
            password: "6782%&123asdcvb!!jzxcqtry56".to_string(),
            email: "derek@email.com".to_string(),
        };
        db.user_add(user.clone()).unwrap();
        assert!(db.user_exists(&user.uuid).unwrap());

        let delete_result = db.user_delete(&user.uuid);
        match delete_result {
            Ok(_) => assert!(!db.user_exists(&user.uuid).unwrap()),
            Err(error) => assert!(false, "Database User Get Error: {:?}", error),
        }

        drop(db);
    }
}
