use log::*;
use screeps::game::{self};

use crate::memory::{memory_utils::clean_memory, room_memory::RoomMemory};

pub fn run_memory_manager() {
    info!("Running memory manager");

    if game::time() % 1000 == 0 {
        clean_memory();
    }

    for room in game::rooms().values() {
        RoomMemory::init_room_memory(room);
    }
}
