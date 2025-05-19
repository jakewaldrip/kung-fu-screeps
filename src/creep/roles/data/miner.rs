use screeps::{Part, Room};

use crate::{creep::roles::roles_api::Roles, memory::creep_memory::CreepMemory, spawn::tier::Tier};

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

    fn get_body(&self, room: &Room) -> Vec<Part> {
        let mut parts: Vec<Part> = Vec::new();

        match Tier::get_for_room(room) {
            Tier::One => {
                parts.extend_from_slice(&vec![Part::Work; 2]);
                parts.extend_from_slice(&vec![Part::Move; 2]);
            }
            Tier::Two => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 1]);
            }
            Tier::Three => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 3]);
            }
            Tier::Four => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 3]);
            }
            Tier::Five => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 3]);
            }
            Tier::Six => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 3]);
            }
            Tier::Seven => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 3]);
            }
            Tier::Eight => {
                parts.extend_from_slice(&vec![Part::Work; 5]);
                parts.extend_from_slice(&vec![Part::Move; 3]);
            }
        };

        parts
    }
}
