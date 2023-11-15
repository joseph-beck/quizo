use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait Model {}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub correct: i32,
    pub wrong: i32,
    pub points: i32,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Answer {
    pub option: i8,
    pub text: String,
    pub correct: bool,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Question {
    pub question: String,
    pub body: String,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Quiz {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Session {
    pub uuid: Uuid,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Room {
    pub uuid: Uuid,
    pub code: i8,
    pub quiz: Quiz,
    pub players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = super::schema::users)]
pub struct User {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub email: String,
}
