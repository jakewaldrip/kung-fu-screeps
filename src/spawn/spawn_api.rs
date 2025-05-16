use std::collections::HashMap;

use screeps::{game, Room};

use crate::{
    creep::roles::roles_api::Roles,
    memory::{creep_memory::CreepMemory, room_memory::RoomState},
};

pub fn get_spawn_limits(_room: &Room, room_state: &RoomState) -> HashMap<Roles, u8> {
    let mut spawn_limits: HashMap<Roles, u8> = HashMap::new();
    match room_state {
        RoomState::BOOTSTRAP => spawn_limits.insert(Roles::Miner, 2),
        RoomState::BASIC => spawn_limits.insert(Roles::Miner, 2),
    };

    spawn_limits
}

pub fn get_living_creep_counts(room: &Room) -> HashMap<Roles, u8> {
    let mut creep_count: HashMap<Roles, u8> = HashMap::new();
    let room_name = room.name().to_string();
    for creep in game::creeps().values() {
        let creep_memory = CreepMemory::get(&creep);
        let home_room = creep_memory.home_room;
        let creep_role = creep_memory.role;

        if room_name == home_room {
            *creep_count.entry(creep_role.clone()).or_insert(0) += 1;
        }
    }

    creep_count
}
