use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_web::web;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use uuid::Uuid;

use crate::{db, server::Disconnect, server::Connect};
use crate::models::NewConversation;
use crate::server;

const HEARBEET: Duration = Duration::from_secs(10000);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10000);
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[derive(Debug)]
pub struct WsChatSession {
    pub id: Uuid,
    pub hb: Instant,
    pub room:String,
    // pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
    pub db_pool: web::Data<DbPool>,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum ChatType {
    STATUS,
    TYPING,
    TEXT,
    CONNECT,
    DISCONNECT,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    pub chat_type: ChatType,
    pub value: Vec<String>,
    pub room_id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub id: Uuid,
}

impl WsChatSession{
    pub fn new(room: String,addr: Addr<server::ChatServer>, db_pool:web::Data<DbPool>) -> WsChatSession{

        WsChatSession { id: Uuid::new_v4(), hb: Instant::now(), room: room, addr: addr, db_pool: db_pool }
    }
}


impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room.clone(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id, room_id: self.room.clone() });
        Running::Stop
    }

    // fn started(&mut self, ctx: &mut Self::Context) {
    //     self.hb(ctx);

    //     let addr = ctx.address();
    //     let _rm = self.room.clone();

    //     self.addr
    //         .send(server::Connect {
    //             addr: addr.recipient(),
    //         })
    //         .into_actor(self)
    //         .then(|res, act, ctx| {
    //             match res {
    //                 Ok(res) => act.id = res,
    //                 _ => ctx.stop(),
    //             }
    //             fut::ready(())
    //         })
    //         .wait(ctx);
    // }

    // fn stopping(&mut self, _: &mut Self::Context) -> Running {
    //     self.addr.do_send(server::Disconnect { id: self.id });
    //     Running::Stop
    // }
}

impl Handler<server::WsMessage> for WsChatSession {
    type Result = ();
    fn handle(&mut self, msg: server::WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let data_json = serde_json::from_str::<ChatMessage>(&text.to_string());
                if let Err(err) = data_json {
                    println!("{err}");
                    println!("Failed to parse message: {text}");
                    return;
                }

                let input = data_json.as_ref().unwrap();
                let input_room_id = db::create_room_id(input.sender_id.clone(), input.receiver_id.clone());
                match &input.chat_type {
                    ChatType::TYPING => {
                        let chat_msg = ChatMessage {
                            chat_type: ChatType::TYPING,
                            value: input.value.to_vec(),
                            id: self.id,
                            room_id: input_room_id.to_owned(),
                            sender_id: input.sender_id.to_string(),
                            receiver_id: input.receiver_id.to_string(),
                        };
                        let msg = serde_json::to_string(&chat_msg).unwrap();
                        self.addr.do_send(server::ClientActorMessage {
                            id: self.id,
                            msg,
                            // room: input_room_id.clone(),
                            room_id: self.room.clone(),
                        })
                    }
                    ChatType::TEXT => {
                        let input = data_json.as_ref().unwrap();
                        let chat_msg = ChatMessage {
                            chat_type: ChatType::TEXT,
                            value: input.value.to_vec(),
                            id: self.id,
                            room_id: input_room_id.to_owned(),
                            sender_id: input.sender_id.to_string(),
                            receiver_id: input.receiver_id.to_string(),
                        };
                        // FIXME

                        let mut conn = self.db_pool.get().unwrap();

                        let new_conversation = NewConversation {
                            sender_id: input.sender_id.to_string(),
                            receiver_id: input.receiver_id.to_string(),
                            message: input.value.join(""),
                        };
            


// #FIXMEEE
                        let _ = db::insert_new_conversation(&mut conn, new_conversation);
                        let msg = serde_json::to_string(&chat_msg).unwrap();
                        println!("{:?}",self.room.clone());
                        self.addr.do_send(server::ClientActorMessage {
                            id: self.id,
                            msg,
                            // room: input_room_id.clone(),
                            room_id: self.room.clone(), 
                            // testing
                        })
                    }
                    _ => {}
                }
            }
            ws::Message::Binary(_) => println!("Unsupported binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsChatSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARBEET, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                act.addr.do_send(server::Disconnect { id: act.id, room_id: act.room.to_owned() });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}