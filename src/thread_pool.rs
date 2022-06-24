use tokio::sync::{oneshot};
use tokio::runtime::{Handle, Runtime};
use futures::executor::block_on;

type CallbackType = Box<dyn Fn() + Send + Sync>;

pub struct ThreadPool {
    pub size: usize,
    index: usize,
    senders: Vec<tokio::sync::mpsc::Sender<CallbackType>>
}

impl ThreadPool {
    pub async fn new(size: usize) -> Self {
        let mut senders: Vec<tokio::sync::mpsc::Sender<CallbackType>> = vec!();
        for _ in 0..size {
            let (sys_tx, mut sys_rx) = tokio::sync::mpsc::channel::<tokio::sync::mpsc::Sender<CallbackType>>(1);

            let rt = Handle::current();
            std::thread::spawn(move || {
                let (tx, mut rx) = tokio::sync::mpsc::channel::<CallbackType>(256);
                rt.block_on(sys_tx.send(tx));

                let async_runtime = Runtime::new().unwrap();
                while let Some(f) = async_runtime.block_on(rx.recv()) {
                    async_runtime.spawn(async move {
                        f();
                    });
                }
            });

            let res = sys_rx.recv().await.unwrap();
            senders.push(res);
        }
        ThreadPool { size, index: 0, senders }
    }

    pub async fn spawn<F>(&mut self, f: F) -> () where F: 'static + Send + Sync + Fn() -> () {
        self.senders.get(self.index).unwrap().send(Box::new(f)).await;
        self.index = (self.index + 1) % self.size;
    }
}
