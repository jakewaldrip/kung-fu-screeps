use screeps::{Part, Room};

use crate::{creep::roles::roles_api::Roles, memory::creep_memory::CreepMemory};

use super::creep_data::CreepData;

pub struct MinerData {}

impl MinerData {
    pub fn get() -> Self {
        Self {}
    }
}

impl CreepData for MinerData {
    fn get_memory(&self, home_room: String) -> CreepMemory {
        CreepMemory {
            home_room,
            role: Roles::Miner,
        }
    }

    fn get_body(&self, _room: &Room) -> Vec<Part> {
        vec![Part::Move, Part::Work, Part::Work]
    }
}
