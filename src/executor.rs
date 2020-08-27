use crate::types::{Action, Direction, Player, Position, Veggie};
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

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

#[derive(Debug)]
pub struct Message {
    pub action: Action,
    pub player: Player,
}
pub struct Executor {
    pub messages: Vec<Arc<Mutex<Message>>>,
    pub game: Game,
    pub receiver: Receiver<Arc<Mutex<Message>>>,
    pub sender: Sender<Arc<Mutex<Message>>>,
}

impl Executor {
    pub fn new(sender: Sender<Arc<Mutex<Message>>>, receiver: Receiver<Arc<Mutex<Message>>>, game: Game) -> Self {
        Executor {
            messages: vec![],
            game,
            receiver,
            sender,
        }
    }
    pub fn start(&mut self) {
        let handler = thread::Builder::new()
            .name("executor".into())
            .spawn(move || {
                self.receiver.iter().for_each(|message| {
                    self.messages.push(message.clone());
                    self.handle(message);
                    thread::sleep(Duration::from_millis(1000))
                })
            });
    }
    pub fn handle(&self, message: Arc<Mutex<Message>>) {
        println!("handled: {:?}", *message);
        let mut message = match message.lock() {
            Ok(m) => m,
            Err(e) => panic!("failed to acquire mutex lock: {:?}", e),
        };
        match message.action.clone() {
            Action::Eat(v) => message.player.eat(v),
            Action::Jump(direction) => message.player.jump(direction),
        }
    }
    pub fn shout(&self, message: Message, s: &Sender<Arc<Mutex<Message>>>) {
        let arc_message = Arc::new(Mutex::new(message));
        s.send(arc_message).expect("Could not send message");
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
