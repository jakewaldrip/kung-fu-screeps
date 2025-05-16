use screeps::{game, Part, Room};

use crate::{creep::roles::roles_api::Roles, memory::creep_memory::CreepMemory};

use super::creep_data::CreepData;

pub struct MinerData {}

impl MinerData {
    pub fn get() -> Self {
        Self {}
    }
}

impl CreepData for MinerData {
    fn get_name(&self, room_name: String) -> String {
        let game_time: String = {
            let game_time_raw = game::time().to_string();
            let split_pos = game_time_raw.char_indices().nth_back(4).unwrap().0;
            game_time_raw[split_pos..].into()
        };

        // TODO handle issue of same role spawning in same room on same tick
        format!("{}_{}_{}", Roles::Miner, room_name, game_time)
    }

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
