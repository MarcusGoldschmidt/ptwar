use crate::game::soldier::Soldier;
use crate::game::{GameId, UnityStats};
use std::collections::HashMap;

pub enum SquadInstruction {}

pub struct Squad {
    id: GameId,
    player_id: GameId,
    name: String,

    stats: UnityStats,
    soldiers: HashMap<GameId, Soldier>,
}

impl Squad {
    pub fn calculate_stats(&mut self) {
        let mut stats = UnityStats::default();

        if self.soldiers.is_empty() {
            self.stats = stats;
            return;
        }

        for (_id, mut soldier) in self.soldiers.iter_mut() {
            soldier.calculate_stats();

            stats.add(soldier.stats());
        }

        let count = self.soldiers.len() as f32;

        // TODO: can use 20% of highest for each stats like HOI4
        stats.speed /= count;
        stats.soft_attack /= count;
        stats.hard_attack /= count;
        stats.defense /= count;
        stats.armor /= count;
        stats.piercing /= count;
        stats.weight /= count;
        stats.build_speed /= count;
        stats.accuracy /= count;
        stats.range /= count;
    }
}
