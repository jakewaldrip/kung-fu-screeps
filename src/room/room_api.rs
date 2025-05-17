use screeps::Room;

use crate::memory::room_memory::{RoomMemory, RoomState};

pub fn set_room_state(room: Room) {
    let mut room_memory = RoomMemory::get(&room);

    // calculate room state here
    let room_state = RoomState::BASIC;

    room_memory.room_state = room_state;
    room_memory.set(&room);
}
