use tokio::sync::mpsc;
use tokio::sync::mpsc::*;
use tokio::task::JoinHandle;
use tokio::sync::{oneshot};

pub trait Actor {
    type Action;

    fn new() -> Self;
    fn reduce(&mut self, action: &Self::Action) -> ();
    fn react(&self, action: &Self::Action) -> ();
}

pub fn start_actor<ActorType: Send, A: 'static + Send + Sync>() -> (oneshot::Receiver<Sender<A>>, JoinHandle<()>)
    where ActorType: Actor<Action = A> {
    let (sys_tx, sys_rx) = oneshot::channel::<Sender<A>>();
    let join_handle = tokio::spawn(async {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<A>(256);
        sys_tx.send(tx);

        let mut actor = ActorType::new();
        while let Some(action) = rx.recv().await {
            actor.reduce(&action);
            actor.react(&action);
        }
        ()
    });
    return (sys_rx, join_handle);
}
