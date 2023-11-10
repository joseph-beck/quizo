use diesel::prelude::*;
use serde::{Deserialize, Serialize};

pub trait Model {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub correct: i32,
    pub wrong: i32,
    pub points: i32,
}

pub struct Answer {
    pub option: i8,
    pub text: String,
    pub correct: bool,
}

pub struct Question {
    pub question: String,
    pub body: String,
    pub answers: Vec<Answer>,
}

pub struct Quiz {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub questions: Vec<Question>,
}

pub struct Lobby {
    pub uuid: String,
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
