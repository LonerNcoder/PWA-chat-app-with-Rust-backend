use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::{
    // collections::{HashMap, HashSet},
    time::SystemTime, env,
};
use dotenv::dotenv;
use uuid::Uuid;
use bcrypt::{DEFAULT_COST,hash, verify, hash_with_salt};
use crate::{models::{Conversation, NewConversation, Room, User, ReturnConversation, UserFriends}, schema::{conversations::sender_id, user_friends}};

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn find_user_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn get_username_by_id(conn: &mut SqliteConnection, id:String) ->String {
    use crate::schema::users;

    let user = users::dsl::users
        .filter(users::id.eq(id.clone()))
        .first::<User>(conn);

    user.unwrap().username
}

// pub fn find_username_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> String {
//     use crate::schema::users::dsl::*;

//     let user = users
//         .filter(id.eq(uid.to_string()))
//         .first::<User>(conn)
//         .unwrap();

//     user.username
// }

pub fn get_conversation_by_room_uid(
    conn: &mut SqliteConnection,
    uid: String,
) -> Result<Option<Vec<Conversation>>, DbError> {
    use crate::schema::conversations;

    let convo = conversations::table
        .filter(conversations::room_id.eq(uid))
        .load::<Conversation>(conn)
        .optional()?;

    Ok(convo)
}

pub fn get_latest_conversation_by_room_uid_and_receiver_uid(
    conn: &mut SqliteConnection,
    room_uid: String,
    receiver_uid: String,
) -> Result<Option<Vec<Conversation>>, DbError> {
    use crate::schema::conversations;

    let convo = conversations::table
        .filter(conversations::room_id.eq(room_uid).and(conversations::receiver_id.eq(receiver_uid.clone())).and(conversations::seen.eq(false)))
        // .filter(conversations::seen.eq(0))
        .load::<Conversation>(conn)
        .optional()?;
    
    diesel::update(conversations::dsl::conversations).filter(conversations::receiver_id.eq(receiver_uid))
        .set(conversations::seen.eq(true))
        .execute(conn)?;

    Ok(convo)
}

pub fn get_room_availability_by_room_id(
    conn: &mut SqliteConnection,
    room_id: String,
) -> bool {
    use crate::schema::rooms;

    let room = rooms::table
        .filter(rooms::room_id.eq(room_id))
        .load::<Room>(conn);

    if room.unwrap().is_empty(){
        return false
    }
    return true
}

pub fn find_user_by_phone(
    conn: &mut SqliteConnection,
    user_phone: String,
) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*; 

    let user = users
        .filter(phone.eq(user_phone))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn get_friends(conn: &mut SqliteConnection, userid:String) -> Result<Option<Vec<UserFriends>>, DbError> {
    use crate::schema::user_friends::dsl::*; 

    let friends = user_friends
        .filter(user_id.eq(userid))
        .load::<UserFriends>(conn)
        .optional()?;

    Ok(friends)
}


// NEED TO IMPLEMENT
// pub fn get_all_rooms(conn: &mut SqliteConnection) -> Result<Vec<RoomResponse>, DbError> {
//     use crate::schema::rooms;
//     use crate::schema::users;

//     let rooms_data: Vec<Room> = rooms::table.get_results(conn)?;
//     let mut ids = HashSet::new();
//     let mut rooms_map = HashMap::new();
//     let data = rooms_data.to_vec();
//     for room in &data {
//         let user_ids = room
//             .participant_ids
//             .split(",")
//             .into_iter()
//             .collect::<Vec<_>>();
//         for id in user_ids.to_vec() {
//             ids.insert(id.to_string());
//         }
//         rooms_map.insert(room.id.to_string(), user_ids.to_vec());
//     }

//     let ids = ids.into_iter().collect::<Vec<_>>();
//     let users_data: Vec<User> = users::table
//         .filter(users::id.eq_any(ids))
//         .get_results(conn)?;
//     let users_map: HashMap<String, User> = HashMap::from_iter(
//         users_data
//             .into_iter()
//             .map(|item| (item.id.to_string(), item)),
//     );

//     let response_rooms = rooms_data.into_iter().map(|room| {
//         let users = rooms_map
//             .get(&room.room_id.to_string())
//             .unwrap()
//             .into_iter()
//             .map(|id| users_map.get(id.to_owned()).unwrap().clone())
//             .collect::<Vec<_>>();
//         return RoomResponse{ room, users };
//     }).collect::<Vec<_>>();
//     Ok(response_rooms)
// }

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str, pn: &str,pw: &str) -> Result<User, DbError> {

    use crate::schema::users::dsl::*;

    let _hashed_pw = hash(nm.clone(), DEFAULT_COST)?;
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        phone: pn.to_owned(),
        password: pw.to_owned(),
        created_at: iso_date(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn create_room_id(senderid: String, receiverid: String) -> String {
    let room_id: String;
    let send_id = senderid.to_owned();
    let recive_id = receiverid.to_owned();
    if (send_id > recive_id){
        room_id = format!("{}&{}",send_id,recive_id).clone();
    }else{
        room_id = format!("{}&{}",recive_id,send_id).clone();
    }

    room_id
}

pub fn insert_new_conversation(
    conn: &mut SqliteConnection,
    new: NewConversation,
) -> Result<Conversation, DbError> {

    use crate::schema::conversations;
    
    let room_Id:String;

    room_Id = create_room_id(new.sender_id.clone(), new.receiver_id.clone());

    let availabe_room = get_room_availability_by_room_id(conn, room_Id.clone());
    use crate::schema::rooms::dsl::*;

    if !availabe_room{
        let new_room = Room {
            room_id: room_Id.clone(),
            last_message : new.message.clone(),
            created_at : iso_date(),
        };
        let friendname: String = get_username_by_id(conn, new.sender_id.clone());
        let new_friend = UserFriends{
            id: Uuid::new_v4().to_string(),
            user_id: new.sender_id.clone(),
            friend_id: new.sender_id.clone(),
            friend_name: friendname.to_owned(),
            created_at: iso_date(),
        };

        diesel::insert_into(rooms)
            .values(new_room)
            .execute(conn)?;
        
        use crate::schema::user_friends::dsl::user_friends;
        diesel::insert_into(user_friends)
            .values(new_friend)
            .execute(conn)?;
    }
        
    diesel::update(rooms.filter(room_id.eq(room_Id.clone())))
            .set(last_message.eq(new.message.clone()))
            .execute(conn)?;

    
    // let hashed_content = hash_with_salt(content,DEFAULT_COST,secret_key)?;    
    let new_conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        room_id: room_Id.clone(),
        sender_id: new.sender_id,
        receiver_id: new.receiver_id,
        content: new.message,
        seen: false,
        created_at: iso_date(),
    };


    diesel::insert_into(conversations::dsl::conversations)
        .values(&new_conversation)
        .execute(conn)?;

    Ok(new_conversation)
}


pub fn search_user_by_phonenumber(conn: &mut SqliteConnection,ph_number:String) -> Result<Option<User>, DbError> {
    use crate::schema::users;
    let user = users::table
                    .filter(users::phone.eq(ph_number))
                    .first::<User>(conn)
                    .optional()?;
    
    Ok(user)
}
