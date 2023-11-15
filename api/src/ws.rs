use actix::{Actor, Context, Handler, MessageResult, Recipient};
use actix_web::{web, App, Error, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

use crate::models::{Room, Session};

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Create,
    Join,
    Leave,
    Message,
    Error,
    Information,
}

#[derive(Serialize, actix::Message)]
#[rtype(result = "()")]
struct Message {
    msg: String,
    msg_type: MessageType,
}

#[derive(actix::Message)]
#[rtype(result = "Uuid")]
pub struct CreateRoom {
    pub session: Session,
}
#[derive(actix::Message)]
#[rtype(result = "Result<(), String>")]
pub struct JoinRoom {
    pub session: Session,
    pub room: Room,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub id: Session,
    pub addr: Recipient<Message>,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub session: Session,
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub session: Session,
}

struct Server {
    sessions: HashMap<Session, Recipient<Message>>,
    rooms: HashMap<Room, HashSet<Session>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    pub fn is_empty(&self, room: &Room) -> bool {
        self.rooms
            .get(&room)
            .map(|sessions| sessions.is_empty())
            .unwrap_or(false)
    }

    pub fn send_message(&self, room: &Room, msg: &Message, skip: &Session) {}

    pub fn leave_rooms(&mut self, session: &Session) {
        let mut rooms: Vec<Room> = Vec::new();
        for (room, sessions) in &mut self.rooms {
            if sessions.remove(&session) {
                rooms.push(room.clone().to_owned());
            }
        }

        for room in rooms {
            let msg = Message {
                msg: "Someone disconncted".to_string(),
                msg_type: MessageType::Leave,
            };
            self.send_message(&room, &msg, &session);
            if self.is_empty(&room) {
                self.rooms.remove(&room);
            }
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let Connect { id, addr } = msg;
        self.sessions.insert(id, addr);
    }
}
impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(
        &mut self,
        Disconnect { session }: Disconnect,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.leave_rooms(&session);
        let _ = self.sessions.remove(&session);
    }
}

impl Handler<Leave> for Server {
    type Result = ();

    fn handle(&mut self, Leave { session }: Leave, _ctx: &mut Self::Context) -> Self::Result {
        self.leave_rooms(&session);
    }
}

impl Handler<Message> for Server {
    type Result = ();

    fn handle(&mut self, msg: Message, _ctx: &mut Self::Context) -> Self::Result {}
}
