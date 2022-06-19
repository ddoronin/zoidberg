mod lib;

use std::time::Duration;
use lib::{Actor, start_actor};
use tokio::runtime::Runtime;

struct EchoActor {
    state: String
}

impl Actor for EchoActor {
    type Action = String;

    fn new() -> Self {
        EchoActor { state: String::from("") }
    }

    fn reduce(&mut self, action: &Self::Action) -> () {
        self.state = format!("echo: {:?}", action);
    }

    fn react(&self, action: &Self::Action) -> () {
        println!("-> {:?}", self.state);
    }
}
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let (actor_receiver, actor_join_handle) = start_actor::<EchoActor, String>();
    let actor_sender = actor_receiver.await.unwrap();

    let actor_sender1 = actor_sender.clone();
    std::thread::spawn(move || {
        block_on(async move {
            actor_sender1.send(String::from("1 foo")).await;
            std::thread::sleep(Duration::from_secs(1));
            actor_sender1.send(String::from("1 bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            actor_sender1.send(String::from("1 foo bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            actor_sender1.send(String::from("1 foooooooooo baaaaaar")).await;
        });
    });

    let actor_sender2 = actor_sender.clone();
    std::thread::spawn(move || {
        block_on(async move {
            actor_sender2.send(String::from("2 foo")).await;
            std::thread::sleep(Duration::from_secs(1));
            actor_sender2.send(String::from("2 bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            actor_sender2.send(String::from("2 foo bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            actor_sender2.send(String::from("2 foooooooooo baaaaaar")).await;
        });
    });

    if let Err(error) = actor_join_handle.await {
        println!("RIP actor :'( {:?}", error);
    }
}
