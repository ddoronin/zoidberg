use zoid::thread_pool::ThreadPool;
use zoid::{Actor, start_actor};
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

use std::collections::HashMap;
use std::time::Duration;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let mut pool = ThreadPool::new(2).await;
    let alice_actor_sender = start_actor::<EchoActor, String>(&mut pool).await;
    let bob_actor_sender = start_actor::<EchoActor, String>(&mut pool).await;
    let tom_actor_sender = start_actor::<EchoActor, String>(&mut pool).await;

    let alice_actor_sender = alice_actor_sender.clone();
    std::thread::spawn(move || {
        block_on(async move {
            alice_actor_sender.send(String::from("alice: foo")).await;
            std::thread::sleep(Duration::from_secs(1));
            alice_actor_sender.send(String::from("alice: bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            alice_actor_sender.send(String::from("alice: foo bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            alice_actor_sender.send(String::from("alice: foooooooooo baaaaaar")).await;
        });
    });

    let bob_actor_sender = bob_actor_sender.clone();
    std::thread::spawn(move || {
        block_on(async move {
            bob_actor_sender.send(String::from("bob: foo")).await;
            std::thread::sleep(Duration::from_secs(1));
            bob_actor_sender.send(String::from("bob: bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            bob_actor_sender.send(String::from("bob: foo bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            bob_actor_sender.send(String::from("bob: foooooooooo baaaaaar")).await;
        });
    });

    let tom_actor_sender = tom_actor_sender.clone();
    std::thread::spawn(move || {
        block_on(async move {
            tom_actor_sender.send(String::from("tom: foo")).await;
            std::thread::sleep(Duration::from_secs(1));
            tom_actor_sender.send(String::from("tom: bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            tom_actor_sender.send(String::from("tom: foo bar")).await;
            std::thread::sleep(Duration::from_secs(1));
            tom_actor_sender.send(String::from("tom: foooooooooo baaaaaar")).await;
        });
    });

    tokio::spawn(async {
        loop {
        }
    }).await;
    // tokio::select! {
    //     Err(error) = alice_actor_join_handle => {
    //         println!("RIP Alice :'( {:?}", error);
    //     },
    //     Err(error) = bob_actor_join_handle => {
    //         println!("RIP Bob :'( {:?}", error);
    //     }
    // };
    // // if let Err(error) = alice_actor_join_handle.await {
    // //     println!("RIP Alice :'( {:?}", error);
    // // }
    // if let Err(error) = bob_actor_join_handle.await {
    //     println!("RIP Bob :'( {:?}", error);
    // }
}
