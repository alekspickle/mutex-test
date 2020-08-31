use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

mod executor;
pub mod types;
use executor::{Executor, Game, Message};

fn main() {
    let (tx, rx) = channel::<Message>();
    let mut e = Executor::new(tx, Game::new());

    println!("Creating peter");
    let peter = e.game.add_player("Peter".into());
    println!("Creating hanna");
    let hanna = e.game.add_player("Hanna".into());

    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);

    // for further scalability we will need an abstraction with 
    // ids to run in main thread loop
    let sender = e.sender.clone();
    let peter_id = peter.id;
    let hanna_id = hanna.id;
    thread::Builder::new().name("main loop".into()).spawn(move || {
        loop {
            sender
                .send(Message::jump_left(peter_id))
                .expect("Could not send message");
            sender
                .send(Message::eat_carrot(hanna_id))
                .expect("Could not send message");
            thread::sleep(Duration::from_millis(1000));
        }
    }).expect("Failed to spawn main loop thread");

    e.start(rx);
    println!("Peter's position is: {:#?}", peter.position);
    println!("Hanna's life level is: {}", hanna.life);
}
