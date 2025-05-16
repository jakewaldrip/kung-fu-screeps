use log::*;
use screeps::game::{self};

use crate::memory::{
    empire_memory::EmpireMemory, memory_api::get_owned_rooms, memory_utils::clean_memory,
    room_memory::RoomMemory,
};

pub fn run_memory_manager() {
    info!("Running memory manager");

    if game::time() % 1000 == 0 {
        clean_memory();
    }

    EmpireMemory::init();
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        RoomMemory::init(&room);
    }
}
