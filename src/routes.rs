
use actix::*;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use diesel::{
    prelude::*,
    
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use uuid::Uuid;

use crate::{db, models::UserFriends};
use crate::models;
use crate::server;
use crate::session;



type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

// #[get("/ws/{sender_id}/{receiver_id}")]
pub async fn chat_server(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<server::ChatServer>>,
    info: web::Path<(String,String)>,
) -> Result<HttpResponse, Error> {
    println!("{:?}",info.0.clone());
    let sender_id = info.0.clone();
    let receiver_id = info.1.clone();
    let new_room_id = db::create_room_id(sender_id, receiver_id);
    

    let ws = session::WsChatSession::new(new_room_id,srv.get_ref().clone(),pool);
    // ws::start(
    //     session::WsChatSession {
    //         id: 0,
    //         hb: Instant::now(),
    //         room: new_room_id.to_string(),
    //         name: Some(new_room_id.to_string()),
    //         addr: srv.get_ref().clone(),
    //         db_pool: pool,
    //     },
    //     &req,
    //     stream
    // )
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

// start with ws//host:port/ws/main
pub async fn _main_room_server(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let res = ws::start(
        session::WsChatSession::new("Main-Home".to_string(),srv.get_ref().clone(),pool),
        &req,
        stream
    )?;
    Ok(res)
}

#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username, &form.phone, &form.password) //add password fields
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_uid(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone: {id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/friends/{user_id}")]
pub async fn get_all_friends(
    pool: web::Data<DbPool>,
    info: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let userid = info.to_owned();
    let friends = web::block(move || {
        let mut conn = pool.get()?;
        db::get_friends(&mut conn, userid)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(data) = friends {
        Ok(HttpResponse::Ok().json(data))
    }else{
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No friends were found")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/conversations/{sender_id}/{receiver_id}")]
pub async fn get_conversation_by_id(
    pool: web::Data<DbPool>,
    info: web::Path<(String,String)>,
) -> Result<HttpResponse, Error> {
    let sender_id = info.0.clone();
    let receiver_id = info.1.clone();
    let room_id:String;
    let created_room_id = db::create_room_id(sender_id, receiver_id);
    room_id = created_room_id.clone();
    let conversations = web::block(move || {
        let mut conn = pool.get()?;
        db::get_conversation_by_room_uid(&mut conn, room_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // let return_convo = models::ReturnConversation::from(conversations.unwrap());
    if let Some(data) = conversations {
        Ok(HttpResponse::Ok().json(data))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No conversation with room_id: {created_room_id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/latest/{sender_id}/{receiver_id}")]
pub async fn get_latest_conversation_by_id(
    pool: web::Data<DbPool>,
    info: web::Path<(String,String)>,
) -> Result<HttpResponse, Error> {
    let sender_id = info.0.clone();
    let receiver_id = info.1.clone();
    let room_id:String;
    let created_room_id = db::create_room_id(sender_id.clone(), receiver_id);
    room_id = created_room_id.clone();
    let conversations = web::block(move || {
        let mut conn = pool.get()?;
        db::get_latest_conversation_by_room_uid_and_receiver_uid(&mut conn, room_id,sender_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(data) = conversations{
        Ok(HttpResponse::Ok().json(data))
    }else{
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No conversation with room_id: {created_room_id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}



#[get("/users/phone/{user_phone}")]
pub async fn get_user_by_phone(
    pool: web::Data<DbPool>,
    phone: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_phone = phone.to_string();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_phone(&mut conn, user_phone)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone: {}", phone.to_string())
            })
            .to_string(),
        );
        Ok(res)
    }
}


#[get("/search/users/phone/{user_phone}")]
pub async fn search_user_by_phone(
    pool: web::Data<DbPool>,
    phone: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_phone = phone.to_string();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::search_user_by_phonenumber(&mut conn, user_phone)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone: {}", phone.to_string())
            })
            .to_string(),
        );
        Ok(res)
    }
}
// #[get("/rooms")]
// pub async fn get_rooms(
//     pool: web::Data<DbPool>,
// ) -> Result<HttpResponse, Error> {
//     let rooms = web::block(move || {
//         let mut conn = pool.get()?;
//         db::get_all_rooms(&mut conn)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     if !rooms.is_empty() {
//         Ok(HttpResponse::Ok().json(rooms))
//     } else {
//         let res = HttpResponse::NotFound().body(
//             json!({
//                 "error": 404,
//                 "message": "No rooms available at the moment.",
//             })
//             .to_string(),
//         );
//         Ok(res)
//     }
// }
