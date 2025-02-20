mod core;
mod event;
mod events;
mod game;
mod system;
mod worker;
mod world;

use crate::system::{GameLoop, TPS};
use std::time::Duration;

pub const DEFAULT_TPS: TPS = 60;

pub struct PTWar {
    pub gloop: GameLoop,
}

impl PTWar {
    pub fn new() -> Self {
        let cpu_cores = num_cpus::get();

        let mut gloop = GameLoop::new(cpu_cores, DEFAULT_TPS);

        PTWar { gloop }
    }

    pub async fn start(&mut self) {
        self.gloop.start().await;
    }
}
