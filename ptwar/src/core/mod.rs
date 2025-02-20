use crate::system::{PtWarServer, Tick};
use crate::worker::TickHandler;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

pub enum SaveInterval {
    None,
    Tick(Tick),
    Time(Duration),
}
pub struct SaveGameSystem {
    save_interval: SaveInterval,
}

#[async_trait]
impl TickHandler for SaveGameSystem {
    async fn handle(&self, tick: Tick, server: Arc<PtWarServer>) {
        let should_save = match self.save_interval {
            SaveInterval::None => false,
            SaveInterval::Tick(interval) => tick % interval == 0,
            SaveInterval::Time(interval) => {
                let stats = server.stats.read().await;

                stats.last_save.map_or(true, |(last_tick, last_save)| {
                    last_save.elapsed() >= interval
                })
            }
        };

        if should_save {
            server.save().await;
        }
    }
}
