use std::collections::{HashMap, HashSet};

use serde_json::json;

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use uuid::Uuid;

use crate::session;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub lobby_id: String,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub room_id: String,
}

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub room_id: String,
}

pub struct ListRooms;

impl actix::Message for ListRooms {
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct ChatServer {
    sessions: HashMap<Uuid, Recipient<WsMessage>>,
    rooms: HashMap<String, HashSet<Uuid>>,
    // rng: ThreadRng,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            // rng: rand::thread_rng(),
        }
    }
    // pub fn new(room_id: String) -> ChatServer {
    //     let mut rooms = HashMap::new();
    //     rooms.insert(room_id, HashSet::new());

    //     Self {
    //         sessions: HashMap::new(),
    //         rooms,
    //         rng: rand::thread_rng()
    //     }
    // }
}

impl ChatServer{

    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_receiver) = self.sessions.get(id_to) {

                let _ = socket_receiver
                    .do_send(WsMessage(message.to_owned()));
            } else{
                println!("{}","couldn't send message");
            }
        }
    }

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = ();

    // fn handle(&mut self, msg: Connect, ctx: &mut Context<Self>) -> Self::Result {
    //     let id = self.rng.gen::<usize>();
    //     self.sessions.insert(id.clone(), msg.addr);
    //     self.rooms
    //         .entry("main".to_string())
    //         .or_insert_with(HashSet::new)
    //         .insert(id);
    //     for room in self.rooms.clone() {
    //         println!("{:?}",room);
    //         self.send_message(room.0.as_str(), &json!({
    //             "value": vec![format!("{}", id)],
    //             "chat_type": session::ChatType::CONNECT
    //         }).to_string(), 0);
    //     }
    //     // self.send_message("main", &json!({
    //     //     "value": vec![format!("{}", id)],
    //     //     "chat_type": session::ChatType::CONNECT
    //     // }).to_string(), 0);
    //     id
    // }

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id.clone())
            .or_insert_with(HashSet::new).insert(msg.self_id);

        self
            .rooms
            .get(&msg.lobby_id.clone())
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));

        self.sessions.insert(
            msg.self_id,
            msg.addr,
        );

        self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| self.send_message(&format!("{} disconnected.", &msg.id), user_id));
            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    //only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }

    // fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
    //     let mut rooms: Vec<String> = vec![];
    //     if self.sessions.remove(&msg.id).is_some() {
    //         for (name, sessions) in &mut self.rooms {
    //             if sessions.remove(&msg.id) {
    //                 rooms.push(name.to_owned());
    //             }
    //         }
    //     }

    //     for room in rooms {
    //         // self.send_message("main", &json!({
    //         //     "room": room,
    //         //     "value": vec![format!("Someone disconnect!")],
    //         //     "chat_type": session::ChatType::DISCONNECT
    //         // }).to_string(), 0);
    //         self.send_message(room.as_str(), &json!({
    //             "room": room,
    //             "value": vec![format!("Someone disconnect!")],
    //             "chat_type": session::ChatType::DISCONNECT
    //         }).to_string(), 0);
    //     }
    // }
}

impl Handler<ClientActorMessage> for ChatServer {
    type Result = ();

    // fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
    //     self.send_message(&msg.room, &msg.msg, msg.id);
    // }

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }
        } else {
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&msg.msg, client));
        }
    }
}

// impl Handler<ListRooms> for ChatServer {
//     type Result = MessageResult<ListRooms>;

//     fn handle(&mut self, _: ListRooms, _: &mut Self::Context) -> Self::Result {
//         let mut rooms = vec![];
//         for key in self.rooms.keys() {
//             rooms.push(key.to_owned());
//         }
//         MessageResult(rooms)
//     }
// }

// impl Handler<Join> for ChatServer {
//     type Result = ();

//     fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
//         let Join {id, name} = msg;
//         let mut rooms = vec![];

//         for (n, sessions) in &mut self.rooms {
//             if sessions.remove(&id) {
//                 rooms.push(n.to_owned());
//             }
//         }

//         for room in rooms {
//             self.send_message(&room, &json!({
//                 "room": room,
//                 "value": vec![format!("Someone disconnect!")],
//                 "chat_type": session::ChatType::DISCONNECT
//             }).to_string(), 0);
//         }

//         self.rooms
//             .entry(name.clone())
//             .or_insert_with(HashSet::new)
//             .insert(id);
//     }
// }