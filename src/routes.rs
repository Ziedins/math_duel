
use std::time::Instant;

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

use crate::db;
use crate::models;
use crate::server;
use crate::session;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

pub async fn math_duel_server(
    req: HttpRequest,
    stream: web::Payload,
    pool: web::Data<DbPool>,
    srv: web::Data<Addr<server::MathDuelServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsMathDuelSession {
            id: 0,
            hb: Instant::now(),
            game: "main".to_string(),
            name: None,
            addr: srv.get_ref().clone(),
            db_pool: pool,
        },
        &req,
        stream
    )
}

#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username)
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
                "message": format!("No user found with id: {id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/moves/{uid}")]
pub async fn get_move_by_id(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let game_id = uid.to_owned();
    let moves = web::block(move || {
        let mut conn = pool.get()?;
        db::get_move_by_game_uid(&mut conn, game_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(data) = moves {
        Ok(HttpResponse::Ok().json(data))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No conversation with game_id: {game_id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/users/username/{user_username}")]
pub async fn get_user_by_username(
    pool: web::Data<DbPool>,
    username_input: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let user_username = username_input.to_string();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_username(&mut conn, user_username)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with username: {}", username_input.to_string())
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/games")]
pub async fn get_games(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let games = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_games(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if !games.is_empty() {
        Ok(HttpResponse::Ok().json(games))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No games available at the moment.",
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/games/user/{uid}")]
pub async fn get_games_by_user(
    pool: web::Data<DbPool>,
    id_input: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id_input.to_owned();
    let games = web::block(move || {
        let mut conn = pool.get()?;
        db::find_games_by_user_id(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    

    Ok(HttpResponse::Ok().json(games))
}

#[get("/games/cards")]
pub async fn get_cards(

) -> Result<HttpResponse, Error> {
    let cards = vec!["-10", "*2", "/2", "+5", "ˆ2"];

    Ok(HttpResponse::Ok().json(cards))
}
