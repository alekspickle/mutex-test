use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

mod executor;
pub mod types;
use executor::{Executor, Game, Message};

fn main() {
    let (tx, rx) = channel::<Message>();
    let mut e = Executor::new(Arc::new(Mutex::new(rx)), Game::new());
    println!("Starting executor");
    e.start();

    println!("Creating peter");
    let peter = e.game.add_player("Peter".into());
    println!("Creating hanna");
    let hanna = e.game.add_player("Hanna".into());

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);

    tx.send(Message::jump_left(peter))
        .expect("Could not send message");
    tx.send(Message::eat_carrot(hanna))
        .expect("Could not send message");

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);
}

pub fn dispatch(s: Sender<Message>, message: Message) {
    s.send(message).expect("Could not send message");
}
