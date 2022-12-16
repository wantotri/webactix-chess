use actix::{Recipient, Actor, Handler, Context};
use std::collections::HashMap;
use uuid::Uuid;
use crate::{
    message::{WsMessage, Disconnect, Connect, ClientActorMessage},
    model::{Game, Color, Status, Winner},
    game::{color, prelude::Level}
};

type Socket = Recipient<WsMessage>;

#[derive(Debug)]
pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,
    rooms: HashMap<Uuid, Game>
}

impl Default for Lobby {
    fn default() -> Self {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new()
        }
    }
}

impl Lobby {
    /// Send message to an actor given the Uuid
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        let game = self.rooms
            .entry(msg.lobby_id)
            .or_insert_with(Game::new);

        if game.players.len() == 0 {
            game.players.insert(msg.self_id, Color::White);
        } else if game.status == Status::GameOver {
            let key = game.players.keys().collect::<Vec<_>>()[0];
            let color = game.players.get(key).unwrap().opposite();
            game.players.insert(msg.self_id, color);
        } else if game.status == Status::Waiting {
            let key = game.players.keys().collect::<Vec<_>>()[0];
            let color = game.players.get(key).unwrap().opposite();
            game.players.insert(msg.self_id, color);
            game.status = Status::Playing;
        } else {
            return ();
        }

        self.rooms.get(&msg.lobby_id).unwrap()
            .players
            .iter()
            .filter(|p| p.0 != &msg.self_id)
            .for_each(|p| self.send_message(&format!("{} just joined!", msg.self_id), p.0));

        self.sessions.insert(msg.self_id, msg.addr);
        self.send_message(&format!("your session_id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get_mut(&msg.room_id).unwrap()
                .players.remove(&msg.id).unwrap();

            if self.rooms.get_mut(&msg.room_id).unwrap().status != Status::GameOver {
                self.rooms.get_mut(&msg.room_id).unwrap().status = Status::Waiting;
            }

            let game = self.rooms.get(&msg.room_id).unwrap();
            let total_players = game.players.len();

            if total_players == 1 {
                game.players
                    .iter()
                    .for_each(|p| { 
                        self.send_message(&format!("{} disconnected.", msg.id), p.0);
                        self.send_message(&format!("status: {}", game.status.as_str()), p.0);
                    });
            } else if total_players == 0 {
                self.rooms.remove(&msg.room_id);
            }
        }
    }
}

impl Handler<ClientActorMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
        if msg.msg.starts_with("\\w") {
            if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
            }

        } else if msg.msg.starts_with("\\get_lobby") {
            let output = format!("{:#?}", &self);
            self.send_message(&output, &msg.id);

        } else if msg.msg.starts_with("\\get_available_games") {
            let mut buf = [b'!'; 36];
            let mut rooms = vec![];
            for (room_id, _game) in self.rooms.iter() {
                if room_id != &msg.room_id {
                    let room_id_str = room_id.simple().encode_lower(&mut buf);
                    rooms.push(room_id_str.to_string());
                }
            }
            if rooms.len() > 0 {
                let message = format!("rooms: {}", rooms.join(" "));
                self.send_message(&message, &msg.id);
            }

        } else if msg.msg.starts_with("\\get_game_stat") {
            let game = self.rooms.get(&msg.room_id).unwrap();
            let status = game.status.as_str();
            let history = game.board.get_history();
            let mut output = format!("game stat: {}\nturn: {}\nhistories:\n", status, history.len());
            for his in history {
                output.push_str(&format!("{}\n", his));
            }
            self.send_message(&output, &msg.id);

        } else if msg.msg.starts_with("\\get_color") {
            let color = self.rooms
                .get(&msg.room_id).unwrap()
                .players.get(&msg.id).unwrap();
            let output = format!("color: {}", color.as_str());
            self.send_message(&output, &msg.id);

        } else if msg.msg.starts_with("\\get_board") {
            let board = self.rooms.get(&msg.room_id).unwrap().board.to_string();
            let output = format!("board:\n{}", board);
            self.send_message(&output, &msg.id);

        } else if msg.msg.starts_with("\\get_status") {
            let status = self.rooms.get(&msg.room_id).unwrap().status.as_str();
            let output = format!("status: {}", status);
            for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                self.send_message(&output, pid);
            }

        } else if msg.msg.starts_with("\\move")
        && self.rooms.get(&msg.room_id).unwrap().status == Status::Playing {
            let moves = msg.msg.split(" ").collect::<Vec<&str>>();
            let game = self.rooms.get_mut(&msg.room_id).unwrap();
            let resp = game.board.moves_piece(moves[1], moves[2]).unwrap();
            let mut output = format!("history: {}", resp);
            let enemy_color = game.players.get(&msg.id).unwrap().opposite();

            if game.board.is_king_checked(enemy_color.opposite().as_color()).unwrap() {
                game.board.undo_moves().unwrap();
                self.send_message("Error: Illegal Moves", &msg.id);
                return ();
            }

            if game.board.is_checkmate(enemy_color.as_color()).unwrap() {
                game.status = Status::GameOver;
                game.winner = Winner::from_color(enemy_color.opposite());
                output.push_str(" Checkmate");

            } else if game.board.is_king_checked(enemy_color.as_color()).unwrap() {
                output.push_str(" Check");

            } else if game.board.is_draw(enemy_color.as_color()).unwrap() {
                game.status = Status::GameOver;
                game.winner = Winner::Draw;
                output.push_str(" Draw");
            }

            for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                self.send_message(&output, pid);
            }

            let board = self.rooms.get(&msg.room_id).unwrap().board.to_string();
            let output = format!("board:\n{}", board);
            for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                self.send_message(&output, pid);
            }

        } else if msg.msg.starts_with("\\castling") {
            let moves= msg.msg.split(" ").collect::<Vec<&str>>();
            match self.rooms
                .get_mut(&msg.room_id).unwrap()
                .board.castling(moves[1], moves[2])
            {
                Ok(output) => {
                    for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                        self.send_message(&format!("history: {}", output), pid);
                    }
                    let board = self.rooms.get(&msg.room_id).unwrap().board.to_string();
                    let output = format!("board:\n{}", board);
                    for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                        self.send_message(&output, pid);
                    }
                },
                Err(err) => self.send_message(&err.to_string(), &msg.id)
            };

        } else if msg.msg.starts_with("\\promote") {
            let cmd = msg.msg.split(" ").collect::<Vec<&str>>();
            let promotion_level = match cmd[2] {
                "queen" => Level::Queen,
                "bishop" => Level::Bishop,
                "knight" => Level::Knight,
                "rook" => Level::Rook,
                _ => Level::Pawn
            };
            match self.rooms
                .get_mut(&msg.room_id).unwrap()
                .board.promote(cmd[1], promotion_level)
            {
                Ok(output) => {
                    for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                        self.send_message(&format!("{}", output), pid);
                    }
                    let board = self.rooms.get(&msg.room_id).unwrap().board.to_string();
                    let output = format!("promoted board:\n{}", board);
                    for (pid, _color) in self.rooms.get(&msg.room_id).unwrap().players.iter() {
                        self.send_message(&output, pid);
                    }
                },
                Err(err) => self.send_message(&err.to_string(), &msg.id)
            }

        } else if msg.msg.starts_with("\\get_possible_moves") {
            let cmd = msg.msg.split(" ").collect::<Vec<&str>>();
            let resp = self.rooms
                .get_mut(&msg.room_id).unwrap()
                .board.get_possible_moves_as_string(cmd[1]);
            let output = format!("possible moves for:\n{}\n{}", cmd[1], resp);
            self.send_message(&output, &msg.id);

        } else if msg.msg.starts_with("\\get_captured") {
            let cmd = msg.msg.split(" ").collect::<Vec<&str>>();
            let color = if cmd[1] == "white" { color::Color::White } else { color::Color::Black };
            let captured = self.rooms
                .get_mut(&msg.room_id).unwrap()
                .board.get_captured(color).unwrap()
                .iter()
                .map(|piece| piece.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            let message = format!("captured_{}: {}", cmd[1], captured);
            self.send_message(&message, &msg.id);

        } else {
            let game = self.rooms.get(&msg.room_id).unwrap();
            game.players.iter()
                .filter(|player| *player.0 != msg.id)
                .for_each(|player| self.send_message(&msg.msg, player.0));
        }
        // Print Lobby instance
        // self.rooms.get(&msg.room_id).unwrap().board.print().unwrap();
    }
}
