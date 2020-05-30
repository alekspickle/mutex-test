use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

mod executor;
pub mod types;
use executor::{Executor, Game, Message};


fn main() {
    let (tx, rx) = channel::<Arc<Mutex<Message>>>();
    let mut game = Game::new();
    let _ = Executor::new(rx, game).start();

    let mut peter = game.add_player("Peter".into());
    let mut hanna = game.add_player("Hanna".into());

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);
    
    hanna.shout(Message::jump_left(peter), tx);
    peter.shout(Message::eat_carrot(hanna), tx);
    
    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);


}
