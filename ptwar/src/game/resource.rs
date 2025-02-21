use crate::common::Static;
use crate::game::GameId;
use std::collections::{BTreeMap, HashMap};

#[derive(Eq, PartialEq, Hash)]
pub enum RawResource {
    Iron,
    Coal,
    Oil,
    Rubber,
    Wood,
    Stone,
    Sulfur,
}

#[derive(Eq, PartialEq, Hash)]
pub enum ProcessedResource {
    Steel,
    Plastic,
    Fuel,
    Rubber,
    Lumber,
    Concrete,
    Gunpowder,
}

#[derive(Eq, PartialEq, Hash)]
pub enum Resource {
    Raw(RawResource),
    Processed(ProcessedResource),
}

pub struct ResourceCount {
    pub resource: Static<Resource>,
    pub max: u32,
    pub amount: u32,
}

impl From<Static<Resource>> for ResourceCount {
    fn from(value: Static<Resource>) -> Self {
        Self {
            resource: value,
            max: 0,
            amount: 0,
        }
    }
}

pub struct ResourceUpdate {
    id: GameId,
    title: String,
    description: &'static str,
    resource: Static<Resource>,
    amount: u32,
}

pub enum StorageUpdateStats {
    Add(ResourceUpdate),
    Sub(ResourceUpdate),
    Percent(ResourceUpdate),
}

// TODO: Implement a proper storage system with better memory usage.
pub struct ResourceStorage {
    resources: HashMap<Static<Resource>, ResourceCount>,
    updates: BTreeMap<GameId, StorageUpdateStats>,
}

impl Default for ResourceStorage {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            updates: Default::default(),
        }
    }
}

impl ResourceStorage {
    pub fn add_update_storage(&mut self, id: GameId, storage: StorageUpdateStats) {
        self.updates.insert(id, storage);

        self.tick();
    }

    pub fn remove(&mut self, id: GameId) {
        self.updates.remove(&id);
    }

    pub fn tick(&mut self) {
        let mut percent = Vec::new();

        for (_id, update) in self.updates.iter() {
            match update {
                StorageUpdateStats::Add(res) => {
                    self.resources
                        .entry(res.resource)
                        .and_modify(|v| {
                            v.amount += res.amount;
                            if v.amount > v.max {
                                v.amount = v.max;
                            }
                        })
                        .or_insert(ResourceCount::from(res.resource));
                }
                StorageUpdateStats::Sub(res) => {
                    self.resources
                        .entry(res.resource)
                        .and_modify(|v| {
                            v.amount -= res.amount;
                            if v.amount > v.max {
                                v.amount = v.max;
                            }
                        })
                        .or_insert(ResourceCount::from(res.resource));
                }
                StorageUpdateStats::Percent(res) => {
                    percent.push(res);
                }
            }
        }

        for update in percent {
            self.resources
                .entry(update.resource)
                .and_modify(|v| {
                    if (0..100).contains(&update.amount) {
                        v.amount += v.amount * (update.amount / 100);

                        if v.amount > v.max {
                            v.amount = v.max;
                        }
                    }
                })
                .or_insert(ResourceCount::from(update.resource));
        }
    }
}
