
pub mod ammo;
pub mod resource;
pub mod soldier;
pub mod squad;

pub type GameId = u64;

pub struct UnityStats {
    speed: f32,
    soft_attack: f32,
    hard_attack: f32,
    defense: f32,
    armor: f32,
    piercing: f32,
    weight: f32,
    build_speed: f32,
    accuracy: f32,
    range: f32,
}

impl UnityStats {
    fn add(&mut self, other: &UnityStats) {
        self.speed += other.speed;
        self.soft_attack += other.soft_attack;
        self.hard_attack += other.hard_attack;
        self.defense += other.defense;
        self.armor += other.armor;
        self.piercing += other.piercing;
        self.weight += other.weight;
        self.build_speed += other.build_speed;
        self.accuracy += other.accuracy;
        self.range += other.range;
    }
}

impl Default for UnityStats {
    fn default() -> Self {
        UnityStats {
            speed: 1.0,
            soft_attack: 1.0,
            hard_attack: 0.0,
            defense: 1.0,
            armor: 0.0,
            piercing: 0.0,
            weight: 0.0,
            build_speed: 0.5,
            accuracy: 0.5,
            range: 0.1,
        }
    }
}
