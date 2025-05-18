use std::collections::HashMap;

use screeps::{game, Room, StructureSpawn};

use crate::{
    creep::roles::roles_api::Roles,
    memory::room_memory::{RoomMemory, RoomState},
};

use super::spawn_utils::get_living_creep_counts;

fn get_spawn_limits(_room: &Room, room_state: &RoomState) -> HashMap<Roles, u8> {
    let mut spawn_limits: HashMap<Roles, u8> = HashMap::new();
    match room_state {
        RoomState::BOOTSTRAP => spawn_limits.insert(Roles::Miner, 2),
        RoomState::BASIC => spawn_limits.insert(Roles::Miner, 2),
    };

    spawn_limits
}

pub fn get_next_role_to_spawn(room: &Room) -> Option<Roles> {
    let room_memory = RoomMemory::get(&room);
    let room_state = room_memory.room_state;

    // Get spawn limits
    let spawn_limits = get_spawn_limits(&room, &room_state);
    let miner_limit = spawn_limits.get(&Roles::Miner).unwrap();

    // Get creep counts
    let creep_counts = get_living_creep_counts(&room);
    let miner_count = creep_counts.get(&Roles::Miner).unwrap_or(&0);

    // Spawn creeps
    if *miner_count < *miner_limit {
        return Some(Roles::Miner);
    }

    None
}

pub fn get_active_spawn_for_room(room: &Room) -> Option<StructureSpawn> {
    for spawn in game::spawns().values() {
        if spawn.room().unwrap().name() == room.name() && spawn.spawning().is_none() {
            return Some(spawn);
        }
    }
    None
}

pub fn get_creep_name_to_spawn(room_name: &str, role: &Roles) -> String {
    let game_time: String = {
        let game_time_raw = game::time().to_string();
        let split_pos = game_time_raw.char_indices().nth_back(4).unwrap().0;
        game_time_raw[split_pos..].into()
    };

    // TODO handle issue of same role spawning in same room on same tick
    format!("{}_{}_{}", role, room_name, &game_time)
}
