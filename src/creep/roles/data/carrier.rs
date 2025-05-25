use screeps::{Part, Room};

use crate::{creep::roles::roles_api::Roles, memory::creep_memory::CreepMemory, spawn::tier::Tier};

use super::creep_data::CreepData;

pub struct CarrierData {}

impl CarrierData {
    pub fn get() -> Self {
        Self {}
    }
}

impl CreepData for CarrierData {
    fn get_memory(&self, home_room: String) -> CreepMemory {
        CreepMemory {
            home_room,
            role: Roles::Carrier,
        }
    }

    fn get_body(&self, room: &Room) -> Vec<Part> {
        let mut parts: Vec<Part> = Vec::new();

        match Tier::get_for_room(room) {
            Tier::One => {
                parts.extend_from_slice(&[Part::Work; 1]);
                parts.extend_from_slice(&[Part::Move; 2]);
                parts.extend_from_slice(&[Part::Carry; 2]);
            }
            Tier::Two => {
                parts.extend_from_slice(&[Part::Work; 2]);
                parts.extend_from_slice(&[Part::Move; 4]);
                parts.extend_from_slice(&[Part::Carry; 3]);
            }
            Tier::Three => {
                parts.extend_from_slice(&[Part::Work; 2]);
                parts.extend_from_slice(&[Part::Move; 7]);
                parts.extend_from_slice(&[Part::Carry; 5]);
            }
            Tier::Four => {
                parts.extend_from_slice(&[Part::Move; 10]);
                parts.extend_from_slice(&[Part::Carry; 10]);
            }
            Tier::Five => {
                parts.extend_from_slice(&[Part::Move; 10]);
                parts.extend_from_slice(&[Part::Carry; 10]);
            }
            Tier::Six => {
                parts.extend_from_slice(&[Part::Move; 10]);
                parts.extend_from_slice(&[Part::Carry; 10]);
            }
            Tier::Seven => {
                parts.extend_from_slice(&[Part::Move; 12]);
                parts.extend_from_slice(&[Part::Carry; 12]);
            }
            Tier::Eight => {
                parts.extend_from_slice(&[Part::Move; 12]);
                parts.extend_from_slice(&[Part::Carry; 12]);
            }
        };

        parts
    }
}
