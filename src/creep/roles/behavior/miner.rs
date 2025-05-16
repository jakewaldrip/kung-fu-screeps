use screeps::Creep;

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
        self.creep.say(&format!("Me Miner"), true).unwrap();
    }
}
