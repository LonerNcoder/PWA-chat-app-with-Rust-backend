use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Clone, Identifiable, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub username: String,
    pub phone: String,
    pub password: String,
    pub created_at: String
}

#[derive(Debug, Clone, Identifiable, PartialEq, Serialize, Deserialize, Queryable, Insertable, Associations)]
#[belongs_to(Room,foreign_key = "room_id")]
#[table_name = "conversations"]
pub struct Conversation {
    pub id: String,
    #[serde(skip_serializing)]
    pub room_id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    #[serde(skip_serializing)]
    pub seen: bool,
    pub created_at: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[table_name = "rooms"]
pub struct Room {
    pub room_id: String,
    pub last_message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, PartialEq)]
#[table_name = "user_friends"]
pub struct UserFriends {
    pub id: String,
    pub user_id: String,
    pub friend_id: String,
    pub friend_name: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewConversation {
    pub sender_id: String,
    pub receiver_id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomResponse {
    pub room: Room,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewParticipent {
    pub id: String,
    pub last_message: String,
    pub participant_ids: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnConversation {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    pub seen: bool,
    pub created_at: String,
}
impl From<Conversation> for ReturnConversation {
    fn from(conv: Conversation) -> Self {
        ReturnConversation { id: conv.id, sender_id: conv.sender_id, receiver_id: conv.receiver_id, content: conv.content,seen: conv.seen, created_at: conv.created_at }
    }
}