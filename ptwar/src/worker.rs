use crate::event::Event;
use crate::system::{PtWarServer, Tick};
use async_trait::async_trait;
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use log::warn;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, RwLock};
use tokio::task::JoinHandle;

#[async_trait]
pub trait TickHandler: Send + Sync {
    async fn handle(&self, _tick: Tick, _server: Arc<PtWarServer>) {
        unimplemented!();
    }
}

#[derive(Clone)]
pub enum WorkerJob {
    Tick(Arc<Box<dyn TickHandler>>),
    Event(Arc<Box<dyn Event>>),
}

pub struct PWorkerManager {
    workers_tx: Vec<Sender<WorkerJob>>,
    idx: usize,
    workers_count: usize,
    buffer_size: usize,
    workers: Vec<PWorker>,
    // Stats
    messages_in_flight: Arc<Mutex<usize>>,
}

impl PWorkerManager {
    pub fn new(state: Arc<PtWarServer>, workers_count: usize, buffer_size: usize) -> Self {
        let messages_in_flight = Arc::new(Mutex::new(0usize));

        let mut workers_tx = Vec::with_capacity(workers_count);
        let mut workers = Vec::with_capacity(workers_count);

        for id in 0..workers_count {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer_size);

            workers_tx.push(tx);

            workers.push(PWorker::new(
                id as u8,
                state.clone(),
                rx,
                messages_in_flight.clone(),
            ))
        }

        PWorkerManager {
            workers_tx,
            idx: 0,
            workers_count,
            buffer_size,
            workers,
            messages_in_flight,
        }
    }

    pub async fn send(&mut self, handler: WorkerJob) {
        let idx = self.idx;
        self.idx = (self.idx + 1) % self.workers_count;

        {
            let mut in_flight = self.messages_in_flight.lock().await;
            *in_flight += 1;
        }

        self.workers_tx[idx].send(handler).await.unwrap();
    }

    pub async fn send_batch(&mut self, handlers: &Vec<WorkerJob>) {
        {
            let mut in_flight = self.messages_in_flight.lock().await;
            *in_flight += handlers.len();
        }

        let worker_count = self.workers_count;

        let batches = handlers.chunks(worker_count);

        let mut futures = FuturesUnordered::new();

        for batch in batches {
            for handle in batch {
                self.idx = (self.idx + 1) % self.workers_count;
                let idx = self.idx.clone();

                let inner_handle = handle.clone();
                let inner_tx = self.workers_tx[idx].clone();

                futures.push(async move {
                    inner_tx.send(inner_handle).await.unwrap();
                });
            }
        }

        self.idx += handlers.len();

        while let Some(_) = futures.next().await {}
    }

    pub async fn wait_all(&self) {
        loop {
            let in_flight = self.messages_in_flight.lock().await;
            if *in_flight == 0 {
                break;
            }

            drop(in_flight);

            tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        }
    }
}

pub enum PWorkerStatus {
    Idle,
    Working,
    Stopped,
}

pub struct PWorker {
    id: u8,
    status: Arc<RwLock<PWorkerStatus>>,
    pub thread: JoinHandle<()>,
}

async fn update_status(ref_status: Arc<RwLock<PWorkerStatus>>, status: PWorkerStatus) {
    let mut w = ref_status.write().await;
    *w = status;
}

impl PWorker {
    pub fn new(
        id: u8,
        server: Arc<PtWarServer>,
        mut receiver: Receiver<WorkerJob>,
        in_flight_count: Arc<Mutex<usize>>,
    ) -> Self {
        let status = Arc::new(RwLock::new(PWorkerStatus::Idle));
        let status_cp = status.clone();

        let thread = tokio::spawn(async move {
            loop {
                update_status(status.clone(), PWorkerStatus::Idle).await;

                let msg = receiver.recv().await;

                update_status(status.clone(), PWorkerStatus::Working).await;

                match msg {
                    Some(msg) => {
                        let tick = server.tick().await;

                        match msg {
                            WorkerJob::Tick(act) => act.handle(tick, server.clone()).await,
                            WorkerJob::Event(act) => {
                                warn!(
                                    "tick: {} Event handling not implemented for event: {}",
                                    tick,
                                    act.get_name()
                                );
                            }
                        };

                        let mut in_flight = in_flight_count.lock().await;
                        *in_flight -= 1;
                    }
                    None => {
                        update_status(status.clone(), PWorkerStatus::Stopped).await;
                        break;
                    }
                }
            }
        });

        PWorker {
            id,
            status: status_cp,
            thread,
        }
    }
}
