use screeps::{game, Creep, OwnedStructureProperties, Room};

use super::creep_memory::CreepMemory;

pub fn get_owned_rooms() -> Vec<Room> {
    game::rooms()
        .values()
        .filter(|room| {
            let controller = room.controller();
            match controller {
                Some(controller) => controller.my(),
                None => false,
            }
        })
        .collect()
}

pub fn get_creeps_in_room(room: &Room) -> Vec<Creep> {
    let room_name = room.name().to_string();
    game::creeps()
        .values()
        .filter(|creep| {
            let creep_memory = CreepMemory::get(creep);
            creep_memory.home_room == room_name
        })
        .collect()
}
