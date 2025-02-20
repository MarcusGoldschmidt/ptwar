use crate::common::Static;
use derivative::Derivative;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum AmmoType {
    Bullet,
    Shell,
    Rocket,
    Grenade,
}

#[derive(Derivative)]
#[derivative(Eq, Hash, PartialEq)]
pub struct AmmoMagDescriptor {
    name: &'static str,
    description: &'static str,

    ammo_type: AmmoType,
    max_count: u16,

    #[derivative(Hash = "ignore")]
    damage: f32,
    #[derivative(Hash = "ignore")]
    weight: f32,
}

pub struct AmmoMag {
    descriptor: Static<AmmoMagDescriptor>,
    count: u16,
}

impl From<Static<AmmoMagDescriptor>> for AmmoMag {
    fn from(value: Static<AmmoMagDescriptor>) -> Self {
        Self {
            descriptor: value,
            count: value.max_count,
        }
    }
}

pub struct AmmoBag {
    inner: HashMap<Static<AmmoMagDescriptor>, Vec<AmmoMag>>,
}

impl Default for AmmoBag {
    fn default() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}

impl AmmoBag {
    pub fn add(&mut self, mag: AmmoMag) {
        let entry = self.inner.entry(mag.descriptor).or_insert_with(Vec::new);
        entry.push(mag);

        if entry.len() > 1 {
            entry.sort_by_key(|i| i.count);
        }
    }

    pub fn get(&mut self, descriptor: Static<AmmoMagDescriptor>) -> Option<AmmoMag> {
        let entry = self.inner.get_mut(&descriptor)?;
        entry.pop()
    }

    pub fn count(&self, descriptor: Static<AmmoMagDescriptor>) -> u16 {
        self.inner
            .get(&descriptor)
            .map_or(0, |v| v.iter().map(|i| i.count).sum())
    }

    pub fn weight(&self) -> f32 {
        self.inner
            .iter()
            .map(|(k, v)| k.weight * v.len() as f32)
            .sum()
    }
}
