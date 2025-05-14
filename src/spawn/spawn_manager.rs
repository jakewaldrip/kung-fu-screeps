use log::info;
use screeps::Room;

use crate::memory::memory_api::get_owned_rooms;

pub fn run_spawn_manager() {
    info!("Running spawn manager");
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        run_spawns_for_room(room);
    }
}

pub fn run_spawns_for_room(room: Room) {
    info!("Running spawn for room {}", room.name());
}
