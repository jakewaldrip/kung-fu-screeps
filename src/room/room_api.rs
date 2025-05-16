use screeps::Room;

use crate::memory::room_memory::{RoomMemory, RoomState};

pub fn set_room_state(room: Room) {
    // calculate room state here
    let room_state = RoomState::BOOTSTRAP;
    RoomMemory::set_room_state(&room, room_state);
}
