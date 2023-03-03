use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};
use uuid::Uuid;
use crate::models::{Move, NewMove, Game, GameResponse, User};

type DbError = Box<dyn std::error::Error + Send + Sync>;

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

pub fn find_user_by_username(
    conn: &mut SqliteConnection,
    user_username: String,
) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;
    let user = users
        .filter(username.eq(user_username))
        .first::<User>(conn)
        .optional()?;
    Ok(user)
}

pub fn find_user_by_uid(conn: &mut SqliteConnection, uid: Uuid) -> Result<Option<User>, DbError> {
    use crate::schema::users::dsl::*;

    let user = users
        .filter(id.eq(uid.to_string()))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}


pub fn get_move_by_game_uid(
    conn: &mut SqliteConnection,
    uid: Uuid,
) -> Result<Option<Vec<Move>>, DbError> {
    use crate::schema::moves;

    let the_move = moves::table
        .filter(moves::game_id.eq(uid.to_string()))
        .load(conn)
        .optional()?;

    Ok(the_move)
}

pub fn get_all_games(conn: &mut SqliteConnection) -> Result<Vec<GameResponse>, DbError> {
    use crate::schema::games;
    use crate::schema::users;

    let games_data: Vec<Game> = games::table.get_results(conn)?;
    let mut ids = HashSet::new();
    let mut games_map = HashMap::new();
    let data = games_data.to_vec();
    for game in &data {
        let mut user_ids = Vec::new();
        user_ids.push(game.user_a_id.clone());
        user_ids.push(game.user_b_id.clone());

        for id in user_ids.to_vec() {
            ids.insert(id.to_string());
        }
        games_map.insert(game.id.to_string(), user_ids.to_vec());
    }

    let ids = ids.into_iter().collect::<Vec<_>>();
    let users_data: Vec<User> = users::table
        .filter(users::id.eq_any(ids))
        .get_results(conn)?;

    let users_map: HashMap<String, User> = HashMap::from_iter(
        users_data
            .into_iter()
            .map(|item| (item.id.to_string(), item)),
    );

    let response_games = games_data.into_iter().map(|game| {
        let users = games_map
            .get(&game.id.to_string())
            .unwrap()
            .into_iter()
            .map(|id| users_map.get(&id.to_owned()).unwrap().clone())
            .collect::<Vec<_>>();
        return GameResponse{ game, users };
    }).collect::<Vec<_>>();
    Ok(response_games)
}

pub fn find_games_by_user_id(conn: &mut SqliteConnection, user_id: Uuid) -> Result<Vec<GameResponse>, DbError> {
    use crate::schema::games;
    use crate::schema::users;

    let user_option:Option<User> = find_user_by_uid(conn, user_id)?;

    let user = match user_option {
        None => return Ok(Vec::new()),
        Some(usr) => usr
    };


    let games_data: Vec<Game> = games::table
    .filter(games::user_a_id.eq(user.id.clone()))
    .load(conn)?;
    let mut ids = HashSet::new();
    let mut games_map = HashMap::new();
    let data = games_data.to_vec();
    for game in &data {
        let mut user_ids = Vec::new();
        user_ids.push(game.user_a_id.clone());
        user_ids.push(game.user_b_id.clone());

        for id in user_ids.to_vec() {
            ids.insert(id.to_string());
        }
        games_map.insert(game.id.to_string(), user_ids.to_vec());
    }

    let ids = ids.into_iter().collect::<Vec<_>>();
    let users_data: Vec<User> = users::table
        .filter(users::id.eq_any(ids))
        .get_results(conn)?;

    let users_map: HashMap<String, User> = HashMap::from_iter(
        users_data
            .into_iter()
            .map(|item| (item.id.to_string(), item)),
    );

    let response_games = games_data.into_iter().map(|game| {
        let users = games_map
            .get(&game.id.to_string())
            .unwrap()
            .into_iter()
            .map(|id| users_map.get(&id.to_owned()).unwrap().clone())
            .collect::<Vec<_>>();
        return GameResponse{ game, users };
    }).collect::<Vec<_>>();
    Ok(response_games)
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        created_at: iso_date(),
    };

    diesel::insert_into(users).values(&new_user).execute(conn)?;

    Ok(new_user)
}

pub fn insert_new_move(
    conn: &mut SqliteConnection,
    new: NewMove,
) -> Result<Move, DbError> {
    use crate::schema::moves::dsl::*;

    let new_move = Move {
        id: Uuid::new_v4().to_string(),
        user_id: new.user_id,
        game_id: new.game_id,
        value: new.value,
        created_at: iso_date(),
    };

    diesel::insert_into(moves)
        .values(&new_move)
        .execute(conn)?;

    Ok(new_move)
}