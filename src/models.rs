use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct User {
    pub id: String,
    pub username: String,
    pub created_at: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Insertable)]
pub struct Move {
    pub id: String,
    pub game_id: String,
    pub user_id: String,
    pub value: String,
    pub created_at: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub user_a_id: String,
    pub user_b_id: String,
    pub goal_a: f32,
    pub goal_b: f32,
    pub current_value: f32,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMove {
    pub user_id: String,
    pub game_id: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub game: Game,
    pub users: Vec<User>,
}
