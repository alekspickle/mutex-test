use crate::types::{Action, Direction, Player, Position, Veggie};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

#[derive(Debug, Default)]
pub struct Game {
    players: HashMap<String, Player>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            players: HashMap::new(),
        }
    }
    pub fn add_player(&mut self, name: String) -> Player {
        let next_id = self.players.len() as u32 + 1;
        let player = Player {
            id: next_id,
            name: name.clone(),
            life: 100u32,
            speed: 0u32,
            position: Position::default(),
        };
        self.players.insert(name, player.clone());
        player
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub action: Action,
    pub player: Player,
}
pub struct Executor {
    pub messages: Vec<Message>,
    pub game: Game,
    pub receiver: Arc<Mutex<Receiver<Message>>>,
}

impl Executor {
    pub fn new(receiver: Arc<Mutex<Receiver<Message>>>, game: Game) -> Self {
        Executor {
            messages: Vec::with_capacity(100),
            game,
            receiver,
        }
    }
    pub fn start(
        &mut self,
    ) -> Result<(), std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>> {
        let handler = thread::Builder::new()
            .name("executor".into())
            .spawn(move || {
                let rec = *self
                    .receiver
                    .lock()
                    .expect("failed to lock on a receiver in executor thread");
                rec.iter().for_each(|message| {
                    let mut messages = *self.messages;
                    self.messages = messages.push(message.clone());
                    self.handle(message);
                })
            })
            .expect("could not spawn executor thread");
        handler.join()
    }
    pub fn handle(&self, message: Message) {
        println!("handled: {:?}", message);
        match message.action.clone() {
            Action::Eat(v) => message.player.eat(v),
            Action::Jump(direction) => message.player.jump(direction),
        }
    }
}

impl Message {
    pub fn jump_left(player: Player) -> Message {
        Message {
            action: Action::Jump(Direction::Left),
            player,
        }
    }
    pub fn jump_right(player: Player) -> Message {
        Message {
            action: Action::Jump(Direction::Right),
            player,
        }
    }
    pub fn jump_up(player: Player) -> Message {
        Message {
            action: Action::Jump(Direction::Up),
            player,
        }
    }
    pub fn jump_down(player: Player) -> Message {
        Message {
            action: Action::Jump(Direction::Down),
            player,
        }
    }
    pub fn eat_carrot(player: Player) -> Message {
        Message {
            action: Action::Eat(Veggie::carrot()),
            player,
        }
    }
    pub fn eat_potato(player: Player) -> Message {
        Message {
            action: Action::Eat(Veggie::potato()),
            player,
        }
    }
    pub fn eat_cucumber(player: Player) -> Message {
        Message {
            action: Action::Eat(Veggie::cucumber()),
            player,
        }
    }
}
