use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Player {
    name: String,
    life: u32,
    speed: u32,
    position: Position,
}

#[derive(Default, Debug)]
struct Position {
     x: u32,
     y: u32,
}

impl Player {
    fn new() -> Player {
        Player {
            name: String::from(""),
            life: 100u32,
            speed: 0u32,
            position: Position::default()
        }
    }
    // fn with_speed(self, s: u32) -> Self {
    //     self.speed = s;
    //     self
    // }
    fn eat(&mut self) {
        self.life += 1;
    }
    fn jump(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.position.y += self.speed,
            Direction::Down => self.position.y -=self.speed,
            Direction::Left => self.position.x -=self.speed,
            Direction::Rigth => self.position.x += self.speed,
        }
    }
    fn send_message(&self, message: Message , s: Sender<Mutex<Arc<Message>>>) {
        let arc_message = Mutex::new(Arc::new(message));
        s.send(arc_message);
    }

}

struct Message{
    action: Action,
    player: Player,
}
struct Executor {
    messages: Vec<Mutex<Arc<Message>>>,
    listener: Receiver<Mutex<Arc<Message>>>
}
enum Action {
    Eat,
    Jump(Direction),
}
enum Direction {
    Up,
    Down,
    Left,
    Rigth,
}


impl Executor {
    fn start(&self) {
        thread::Builder::new().name("executor".into()).spawn(move ||{
            loop {
                let message = self.listener.recv().expect("failed to receive a message");
                self.handle(message);
                thread::sleep(Duration::from_secs(1))
            }
        });
    }
    fn handle(&self, message: Mutex<Arc<Message>>){
        message.lock().expect("failed to unlock the mutex");
        match message.action {
            Action::Eat => message.player.eat(),
            Action::Jump(direction) => message.player.jump(direction),
        }
    }
}

fn main() {
    let (rx, tx) = channel::<Mutex<Arc<Message>>>();
    let mut player1 = Player::new();
    let mut player2 = Player::new();
    // let mut player = Player::new().with_speed(100);
    
    player1.jump(Direction::Up);

}
