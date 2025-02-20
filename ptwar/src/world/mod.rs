use crate::system::Tick;
use std::time::Instant;

pub struct World {
    pub last_save: Option<(Tick, Instant)>,
}
