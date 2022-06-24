use std::ops::Deref;
use tokio::sync::mpsc;
use tokio::sync::mpsc::*;
use tokio::task::JoinHandle;
use tokio::sync::{oneshot};
use tokio::runtime::{Handle, Runtime};
pub mod thread_pool;
use crate::thread_pool::ThreadPool;

pub trait Actor {
    type Action;

    fn new() -> Self;
    fn reduce(&mut self, action: &Self::Action) -> ();
    fn react(&self, action: &Self::Action) -> ();
}

pub async fn start_actor<ActorType: 'static + Send, A: 'static + Send + Sync>(pool: &mut ThreadPool) -> Sender<A>
    where ActorType: Actor<Action = A> {

    let (sys_tx, mut sys_rx) = tokio::sync::mpsc::channel::<Sender<A>>(256);
    pool.spawn(move || {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<A>(256);
        let mut actor = ActorType::new();
        let sys_tx = sys_tx.clone();
        Handle::current().spawn(async move {
            sys_tx.send(tx).await;
            while let Some(action) = rx.recv().await {
                actor.reduce(&action);
                actor.react(&action);
            };
        });
    }).await;

    while let sender = sys_rx.recv().await {
        if sender.is_some() {
            return sender.unwrap();
        }
    }
    panic!("IM IN BIG (_j_) PANIC");
}
