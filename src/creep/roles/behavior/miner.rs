use screeps::{Creep, SharedCreepProperties};

use super::creep_behavior::CreepBehavior;

pub struct MinerBehavior {
    creep: Creep,
}

impl MinerBehavior {
    pub fn get(creep: Creep) -> Self {
        Self { creep }
    }
}

impl CreepBehavior for MinerBehavior {
    fn say(&self) -> () {
        self.creep
            .say(&format!("My name is {}", self.creep.name()), true)
            .unwrap();
    }
}
