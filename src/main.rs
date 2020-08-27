use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

mod executor;
pub mod types;
use executor::{Executor, Game, Message};

fn main() {
    let (tx, rx) = channel::<Arc<Mutex<Message>>>();
    let mut e = Executor::new(tx, rx, Game::new());
    println!("Starting executor");
    e.start();
    
    println!("Creating peter");
    let peter = e.game.add_player("Peter".into());
    println!("Creating hanna");
    let hanna = e.game.add_player("Hanna".into());

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);

    e.shout(Message::jump_left(peter.clone()), &tx);
    e.shout(Message::eat_carrot(hanna.clone()), &tx);

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);
}
