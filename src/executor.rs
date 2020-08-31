use crate::types::{Action, Direction, Player, Position, Veggie};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Game {
    pub players: HashMap<u32, Player>,
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
    pub messages: Vec<Message>,
    pub game: Game,
    pub sender: Sender<Message>,
}

impl Executor {
    pub fn new(sender: Sender<Message>, game: Game) -> Self {
        Executor {
            messages: Vec::new(),
            game,
            sender,
        }
    }
    pub fn start(&mut self, r: Receiver<Message>) {
        loop {
            r.iter().for_each(|message| {
                self.messages.push(message.clone());
                self.handle(self.messages.clone());
            });
            thread::sleep(Duration::from_millis(1000))
        }
    }
    fn handle(&mut self, messages: Vec<Message>) {
        messages.into_iter().for_each(|m| {
            let message = m.clone();

            let player = self.game.players.get_mut(&message.player).unwrap();
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
