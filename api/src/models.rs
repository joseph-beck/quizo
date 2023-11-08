use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

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
    pub answers: Vec<Answer>
}

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset)]
pub struct Quiz {
    pub id: i32,
    pub questions: Vec<Question>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lobby {
    pub id: i8,
    pub quiz: Quiz,
    pub players: Vec<Player>,
}
