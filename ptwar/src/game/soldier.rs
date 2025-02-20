use crate::game::ammo::{AmmoBag, AmmoType};
use crate::game::{GameId, UnityStats};

pub struct Helmet {
    id: GameId,
    name: &'static str,

    defense: f32,
    amor: f32,
    weight: f32,
}

pub struct Armor {
    id: GameId,
    name: &'static str,

    defense: f32,
    amor: f32,
    weight: f32,
}

pub struct Weapon {
    id: GameId,
    name: &'static str,

    soft_attack: f32,
    hard_attack: f32,
    piercing: f32,
    accuracy: f32,
    fire_rate: f32,
    weight: f32,
    range: f32,
    ammo_type: AmmoType,
}

pub enum SoldierModifier {
    Accuracy(f32),
    Speed(f32),
    Health(f32),
    Weight(f32),
}

impl SoldierModifier {
    pub fn apply(&self, soldier: &mut Soldier) {
        match self {
            SoldierModifier::Accuracy(value) => soldier.stats.soft_attack *= value,
            SoldierModifier::Speed(value) => soldier.stats.speed *= value,
            SoldierModifier::Health(value) => soldier.max_hp *= (*value as u16),
            SoldierModifier::Weight(value) => soldier.stats.weight *= value,
        }
    }
}

pub struct SpecialKit {
    id: GameId,
    name: &'static str,
    description: &'static str,
    modifiers: &'static [SoldierModifier],
    weight: f32,
}

pub struct Soldier {
    id: GameId,
    squad_id: GameId,

    name: String,
    age: u8,
    max_hp: u16,
    hp: u16,
    level: u8,
    experience: u32,
    max_weight: f32,

    // Gear
    helmet: Option<Helmet>,
    armor: Option<Armor>,
    weapon: Option<Weapon>,
    special_kit1: Option<SpecialKit>,
    special_kit2: Option<SpecialKit>,

    base_accuracy: f32,
    base_speed: f32,

    ammo_bag: AmmoBag,

    stats: UnityStats,
}

const MAX_LEVEL: u8 = 100;
const BASE_BUILD_SPEED_PER_LEVEL: f32 = 0.05_f32;
const BASE_MOVEMENT_SPEED_PER_LEVEL: f32 = 0.05_f32;
const BASE_ACCURACY_PER_LEVEL: f32 = 0.05_f32;
const WEIGHT_DECAY_FACTOR: f32 = 0.5_f32;

impl Soldier {
    pub fn calculate_stats(&mut self) {
        let mut stats = UnityStats::default();
        stats.accuracy = self.base_accuracy;
        stats.speed = self.base_speed;

        {
            let level_bonus = (self.level - 1) as f32;

            stats.build_speed += level_bonus * BASE_BUILD_SPEED_PER_LEVEL;
            stats.speed += level_bonus * BASE_MOVEMENT_SPEED_PER_LEVEL;
        }

        if let Some(helmet) = &self.helmet {
            stats.defense += helmet.defense;
            stats.armor += helmet.amor;
            stats.weight += helmet.weight;
        }

        if let Some(armor) = &self.armor {
            stats.defense += armor.defense;
            stats.armor += armor.amor;
            stats.weight += armor.weight;
        }

        if let Some(weapon) = &self.weapon {
            stats.soft_attack += weapon.soft_attack;
            stats.hard_attack += weapon.hard_attack;
            stats.piercing += weapon.hard_attack;
            stats.accuracy += weapon.accuracy;
            stats.weight += weapon.weight;
            stats.range += weapon.range;
        }

        if let Some(kit) = &self.special_kit1 {
            for modifier in kit.modifiers {
                modifier.apply(self);
            }
        }

        if let Some(kit) = &self.special_kit2 {
            for modifier in kit.modifiers {
                modifier.apply(self);
            }
        }

        stats.weight += self.ammo_bag.weight();

        // Exponential decrease speed based on weight
        stats.speed *= (-WEIGHT_DECAY_FACTOR * (stats.weight / self.max_weight)).exp();
    }

    pub fn stats(&self) -> &UnityStats {
        &self.stats
    }
}
