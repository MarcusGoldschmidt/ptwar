use crate::event::Event;
use crate::system::SOrder::{First, Second};
use crate::worker::{PWorkerManager, TickHandler, WorkerJob};
use crate::world::World;
use log::{info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};

pub struct ServerStats {
    pub tick: u128,
    pub last_tick: Instant,
    pub last_save: Option<(u128, Instant)>,
}

impl Default for ServerStats {
    fn default() -> Self {
        Self {
            tick: 0,
            last_tick: Instant::now(),
            last_save: None,
        }
    }
}

pub struct PtWarServer {
    pub tick: Arc<RwLock<Tick>>,
    pub events_queue: Arc<Mutex<Vec<Box<dyn Event>>>>,
    pub world: Arc<RwLock<World>>,
    pub stats: Arc<RwLock<ServerStats>>,
}

impl PtWarServer {
    pub fn new() -> Self {
        Self {
            tick: Arc::new(RwLock::new(0)),
            events_queue: Default::default(),
            world: Arc::new(RwLock::new(World { last_save: None })),
            stats: Default::default(),
        }
    }

    pub async fn tick(&self) -> Tick {
        self.tick.read().await.clone()
    }

    pub async fn next_tick(&self) {
        let mut tick = self.tick.write().await;
        *tick += 1;
    }

    pub async fn add_event(&self, event: impl Event + 'static) {
        let mut queue = self.events_queue.lock().await;

        queue.push(Box::new(event));
    }

    // TODO: implement save method
    pub async fn save(&self) {
        let mut world = self.world.write().await;

        self.stats.write().await.last_save = Some((self.tick().await, Instant::now()));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum SOrder {
    First,
    Second,
    Third,
    Last,
}

impl SOrder {
    fn order() -> impl Iterator<Item = &'static SOrder> {
        static ORDERS: [SOrder; 4] = [First, Second, SOrder::Third, SOrder::Last];
        ORDERS.iter()
    }
}

pub type TPS = u8;

pub type Tick = u128;

pub struct GameLoop {
    server: Arc<PtWarServer>,
    tps: TPS,
    tick_duration: Duration,
    last_tick: Instant,
    workers_count: usize,
    manager: PWorkerManager,
    systems: HashMap<SOrder, Vec<Arc<Box<dyn TickHandler>>>>,
}

impl GameLoop {
    pub fn new(workers_count: usize, tps: TPS) -> Self {
        let server = Arc::new(PtWarServer::new());

        let manager = PWorkerManager::new(server.clone(), workers_count, 2048);

        let tick_duration = Duration::from_secs(1) / tps as u32;

        GameLoop {
            server,
            tps,
            tick_duration,
            last_tick: Instant::now(),
            workers_count,
            manager,
            systems: Default::default(),
        }
    }

    pub fn tick_duration(&self) -> Duration {
        self.tick_duration
    }

    pub fn workers_count(&self) -> usize {
        self.workers_count
    }

    pub async fn start(&mut self) {
        info!(
            "Starting game loop tps: {}, tick on: {}ms",
            self.tps,
            self.tick_duration.as_millis()
        );

        loop {
            let elapsed = Instant::now().duration_since(self.last_tick);

            if elapsed < self.tick_duration {
                continue;
            }

            let start_process = Instant::now();

            // Process tick
            for x in SOrder::order() {
                if let Some(systems) = self.systems.get(x) {
                    for system in systems {
                        self.manager.send(WorkerJob::Tick(system.clone())).await;
                    }
                }
            }

            self.manager.wait_all().await;

            // TODO: tracemeter time spent on each system
            {
                // Send events
                let mut events_queue = self.server.events_queue.lock().await;

                for event in events_queue.drain(..) {
                    self.manager.send(WorkerJob::Event(Arc::new(event))).await;
                }
            }

            // Wait for all events to be processed
            self.manager.wait_all().await;

            let elapsed = start_process.elapsed();

            if elapsed > self.tick_duration {
                warn!(
                    "Tick took longer than expected: got {}ms of {}ms range",
                    elapsed.as_millis(),
                    self.tick_duration.as_millis()
                );
            }

            self.last_tick = Instant::now();
            self.server.next_tick().await;
        }
    }

    pub fn add_system(&mut self, order: SOrder, system: impl TickHandler + 'static) {
        let systems = self.systems.entry(order).or_insert_with(Vec::new);
        systems.push(Arc::new(Box::new(system)));
    }
}
