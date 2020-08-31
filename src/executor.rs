use crate::types::{Action, Direction, Player, Position, Veggie};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Default)]
pub struct Game {
    players: HashMap<u32, Player>,
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
        self.players.insert(player.id, player.clone());
        player
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub action: Action,
    pub player: u32,
}
pub struct Executor {
    // pub messages: Arc<Mutex<Vec<Message>>>,
    pub game: Game,
    pub receiver: Arc<Mutex<Receiver<Message>>>,
}

impl Executor {
    pub fn new(receiver: Arc<Mutex<Receiver<Message>>>, game: Game) -> Self {
        // let messages = Arc::new(Mutex::new(Vec::with_capacity(100)));
        Executor {
            // messages,
            game,
            receiver,
        }
    }
    pub fn start(
        &mut self,
    ) -> Result<(), std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>> {
        // let handler = thread::Builder::new()
        //     .name("executor".into())
        //     .spawn(move || {
        let rec = &*self
            .receiver
            .lock()
            .expect("failed to lock on a receiver in executor thread");
        // let mut messages = self
        // .messages
        // .lock()
        // .expect("failed to lock on a receiver in executor thread");
        let mut messages = Vec::new();
        rec.iter().for_each(|message| {
            messages.push(message.clone());
        });
        // })
        // .expect("could not spawn executor thread");
        // handler.join()

        self.handle(messages);
        Ok(())
    }
    pub fn handle(&mut self, messages: Vec<Message>) {
        messages.into_iter().for_each(|m|{

            println!("handled: {:?}", m);
            let mut message = m.clone();
            
            let mut player = self.game.players.get_mut(&message.player).unwrap();
            match message.action.clone() {
                Action::Eat(v) => player.eat(v),
                Action::Jump(direction) => player.jump(direction),
            }
        })
        }
}

impl Message {
    pub fn jump_left(player: u32) -> Message {
        Message {
            action: Action::Jump(Direction::Left),
            player,
        }
    }
    pub fn jump_right(player: u32) -> Message {
        Message {
            action: Action::Jump(Direction::Right),
            player,
        }
    }
    pub fn jump_up(player: u32) -> Message {
        Message {
            action: Action::Jump(Direction::Up),
            player,
        }
    }
    pub fn jump_down(player: u32) -> Message {
        Message {
            action: Action::Jump(Direction::Down),
            player,
        }
    }
    pub fn eat_carrot(player: u32) -> Message {
        Message {
            action: Action::Eat(Veggie::carrot()),
            player,
        }
    }
    pub fn eat_potato(player: u32) -> Message {
        Message {
            action: Action::Eat(Veggie::potato()),
            player,
        }
    }
    pub fn eat_cucumber(player: u32) -> Message {
        Message {
            action: Action::Eat(Veggie::cucumber()),
            player,
        }
    }
}
