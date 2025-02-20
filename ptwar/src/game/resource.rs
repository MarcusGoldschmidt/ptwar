use crate::common::Static;
use crate::game::GameId;
use std::collections::HashMap;

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

pub struct ResourceStorage {
    resources: HashMap<Static<Resource>, ResourceCount>,

    updates_tick: HashMap<GameId, ResourceUpdate>,
    updates_storage: HashMap<GameId, ResourceUpdate>,
    updates_percent: HashMap<GameId, ResourceUpdate>,
}

impl Default for ResourceStorage {
    fn default() -> Self {
        Self {
            resources: Default::default(),
            updates_tick: Default::default(),
            updates_storage: Default::default(),
            updates_percent: Default::default(),
        }
    }
}

impl ResourceStorage {
    pub fn add_update_storage(&mut self, storage: ResourceUpdate) {
        let resource = storage.resource;
        let amount = storage.amount;

        self.updates_storage.insert(storage.id, storage);

        self.resources
            .entry(resource)
            .and_modify(|v| {
                v.max += amount;
            })
            .or_insert(ResourceCount::from(resource));
    }

    pub fn add_update_tick(&mut self, tick: ResourceUpdate) {
        self.updates_tick.insert(tick.id, tick);
    }

    pub fn remove(&mut self, id: GameId) {
        self.updates_tick.remove(&id);
        self.updates_percent.remove(&id);

        self.updates_storage.remove(&id).map(|v| {
            self.resources.entry(v.resource).and_modify(|v| {
                v.amount -= v.amount;
            });
        });
    }

    pub fn tick(&mut self) {
        for (_id, update) in self.updates_tick.iter() {
            self.resources
                .entry(update.resource)
                .and_modify(|v| {
                    v.amount += update.amount;
                    if v.amount > v.max {
                        v.amount = v.max;
                    }
                })
                .or_insert(ResourceCount::from(update.resource));
        }

        for (_id, update) in self.updates_percent.iter() {
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
