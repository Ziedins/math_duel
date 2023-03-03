use std::collections::{HashMap, HashSet};

use serde_json::json;

use actix::prelude::*;
use rand::{self, rngs::ThreadRng, Rng};
use crate::session;


#[derive(Message)]
#[rtype(result = "()")]
pub struct Move(pub String);

#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Move>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMove {
    pub id: usize,
    pub value: String,
    pub game: String,
}

pub struct ListGames;

impl actix::Message for ListGames{
    type Result = Vec<String>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub id: usize,
    pub name: String,
}


#[derive(Debug)]
pub struct MathDuelServer {
    sessions: HashMap<usize, Recipient<Move>>,
    games: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl MathDuelServer {
    pub fn new() -> MathDuelServer {
        let mut games = HashMap::new();
        games.insert("main".to_string(), HashSet::new());

        Self {
            sessions: HashMap::new(),
            games,
            rng: rand::thread_rng()
        }
    }

    fn send_message(&self, game: &str, value: &str, skip_id: usize) {
        if let Some(sessions) = self.games.get(game) {
            for id in sessions {
                if *id != skip_id {
                    if let Some(addr) = self.sessions.get(id) {
                        addr.do_send(Move(value.to_owned()));
                    }
                }
            }
        }
    }
}

impl Actor for MathDuelServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for MathDuelServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let id = self.rng.gen::<usize>();
        self.sessions.insert(id, msg.addr);
        self.games
            .entry("main".to_string())
            .or_insert_with(HashSet::new)
            .insert(id);
        self.send_message("main", &json!({
            "value": vec![format!("{}", id)],
            "chat_type": session::MathDuelType::CONNECT
        }).to_string(), 0);
        id
    }
}

impl Handler<Disconnect> for MathDuelServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        let mut games: Vec<String> = vec![];
        if self.sessions.remove(&msg.id).is_some() {
            for (name, sessions) in &mut self.games {
                if sessions.remove(&msg.id) {
                    games.push(name.to_owned());
                }
            }
        }

        for game in games {
            self.send_message("main", &json!({
                "game": game,
                "value": vec![format!("Someone disconnect!")],
                "chat_type": session::MathDuelType::DISCONNECT
            }).to_string(), 0);
        }
    }
}

impl Handler<ClientMove> for MathDuelServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMove, _: &mut Self::Context) -> Self::Result {
        self.send_message(&msg.game, &msg.value, msg.id);
    }
}

impl Handler<ListGames> for MathDuelServer {
    type Result = MessageResult<ListGames>;

    fn handle(&mut self, _: ListGames, _: &mut Self::Context) -> Self::Result {
        let mut games = vec![];
        for key in self.games.keys() {
            games.push(key.to_owned());
        }
        MessageResult(games)
    }
}

impl Handler<Join> for MathDuelServer {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Self::Context) -> Self::Result {
        let Join {id, name} = msg;
        let mut games = vec![];

        for (n, sessions) in &mut self.games {
            if sessions.remove(&id) {
                games.push(n.to_owned());
            }
        }

        for game in games {
            self.send_message(&game, &json!({
                "game": game,
                "value": vec![format!("Someone disconnect!")],
                "chat_type": session::MathDuelType::DISCONNECT
            }).to_string(), 0);
        }

        self.games
            .entry(name.clone())
            .or_insert_with(HashSet::new)
            .insert(id);
    }
}
