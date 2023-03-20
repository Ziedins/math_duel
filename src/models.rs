use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::*;
use rand::seq::SliceRandom;

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
    pub operator: String,
    pub term: String,
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
    pub operator: String,
    pub term: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub game: Game,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub operator: char,
    pub term: i32,
    pub game_id: String,
}

impl Card {
    pub fn new_card(game_id: &String) -> Card
    {
        let terms = [1,2,3,4,5,6,7,8,9,10];
        let operators = ['+', '-', '*', '/', '^'];

        Card {
            id: Uuid::new_v4().to_string(),
            operator: (operators.choose(&mut rand::thread_rng()).unwrap().clone()),
            term: (terms.choose(&mut  rand::thread_rng()).unwrap().clone()),
            game_id: game_id.to_owned()
        }
    }

    pub fn card_hand(hand_size: usize, game_id: String) -> Vec<Card>
    {
        let mut cards = Vec::with_capacity(hand_size);
        let mut n = 0;
        while n < hand_size {
            cards.push(self::Card::new_card(&game_id));
            n += 1;
        }

        cards
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Operator {
    Plus,
    Minus,
    Multiplication,
    Division,
    Exponentiation
}

impl Operator {

    pub fn operator_from_string(symbol: &str) -> Operator
    {
        match symbol {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "^" => Operator::Exponentiation,
            _ => panic!("Non covered symbol provided, please provide one of theese symbols : +,-.*,/,^"),
        }
    }
}


