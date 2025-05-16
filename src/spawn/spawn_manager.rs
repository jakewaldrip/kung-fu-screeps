use log::info;
use screeps::{game, Room};

use crate::{
    creep::roles::roles_api::Roles,
    memory::{memory_api::get_owned_rooms, room_memory::RoomMemory},
    spawn::spawn_api::{get_living_creep_counts, get_spawn_limits},
};

pub fn run_spawn_manager() {
    info!("Running spawn manager");
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        run_spawns_for_room(room);
    }
}

pub fn run_spawns_for_room(room: Room) {
    info!("Running spawn for room {}", room.name());
    let room_memory = RoomMemory::get(&room);
    let room_state = room_memory.room_state;
    let _spawn_for_room = game::spawns().get("Spawn1".into()).unwrap();

    // Get spawn limits
    let spawn_limits = get_spawn_limits(&room, &room_state);
    let miner_limit = spawn_limits.get(&Roles::Miner).unwrap();

    // Get creep counts
    let creep_counts = get_living_creep_counts(&room);
    let miner_count = creep_counts.get(&Roles::Miner).unwrap_or(&0);

    // TODO, move to get next role concept
    // Spawn creeps
    if *miner_count < *miner_limit {
        // get body
        // get memory
        // get name
        //
        // spawn miner
    }
}
