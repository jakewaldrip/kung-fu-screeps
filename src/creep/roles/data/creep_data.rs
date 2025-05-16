use screeps::{Part, Room};

use crate::memory::creep_memory::CreepMemory;

pub trait CreepData {
    fn get_name(&self, room_name: String) -> String;
    fn get_memory(&self, home_room: String) -> CreepMemory;
    fn get_body(&self, room: &Room) -> Vec<Part>;
}
