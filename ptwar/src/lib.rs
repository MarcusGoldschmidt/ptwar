pub mod common;
pub mod core;
pub mod event;
pub mod events;
pub mod game;
pub mod system;
pub mod worker;
pub mod world;

use crate::system::{GameLoop, TPS};
use log::info;

use sysinfo::System;

pub const DEFAULT_TPS: TPS = 60;

pub struct PTWar {
    pub gloop: GameLoop,
}

impl PTWar {
    pub fn new() -> Self {
        let cpu_cores = num_cpus::get();

        let gloop = GameLoop::new(cpu_cores, DEFAULT_TPS);

        PTWar { gloop }
    }

    pub async fn start(&mut self) {
        let mut sys = System::new_all();

        // First we update all information of our `System` struct.
        sys.refresh_all();

        let memory = sysinfo::get_current_pid()
            .ok()
            .and_then(|pid| sys.process(pid))
            .map(|process| process.memory());

        if let Some(memory) = memory {
            info!("memory usage before start: {}Mb", memory / (1024 * 1024));
        }

        self.gloop.start().await;
    }
}
