use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};

mod executor;
pub mod types;
use executor::{Executor, Game, Message};

fn main() {
    let (tx, rx) = channel::<Message>();
    let rec = Arc::new(Mutex::new(rx));
    let mut e = Executor::new(rec, Game::new());
    println!("Starting executor");
    let _join = e.start();

    println!("Creating peter");
    let peter = e.game.add_player("Peter".into());
    println!("Creating hanna");
    let hanna = e.game.add_player("Hanna".into());

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);

    tx.send(Message::jump_left(peter.id))
        .expect("Could not send message");
    tx.send(Message::eat_carrot(hanna.id))
        .expect("Could not send message");

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);
}

