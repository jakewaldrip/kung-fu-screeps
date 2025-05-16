use rand::Rng;
use screeps::{game, Part, Room};

use crate::creep::roles::roles_api::Roles;

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

        let mut rng = rand::thread_rng();
        let rand_num = rng.gen_range(1..=999).to_string();

        format!("{}_{}_{}_{}", Roles::Miner, room_name, game_time, rand_num)
    }

    fn get_memory(&self, room_name: String) -> crate::memory::creep_memory::CreepMemory {
        todo!()
    }

    fn get_body(&self, room: &Room) -> Vec<Part> {
        todo!()
    }
}
