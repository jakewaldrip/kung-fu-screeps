use screeps::Room;

use crate::{
    creep::roles::roles_api::Roles,
    memory::room_memory::{RoomMemory, RoomState},
    spawn::spawn_utils::get_living_creep_counts,
};

pub fn set_room_state(room: &Room) {
    let mut room_memory = RoomMemory::get(room);

    let room_state = calculate_room_state(room);

    room_memory.room_state = room_state;
    room_memory.set(room);
}

fn calculate_room_state(room: &Room) -> RoomState {
    let creep_counts = get_living_creep_counts(room);
    let miner_count = creep_counts.get(&Roles::Miner).unwrap_or(&0);

    if *miner_count < 1 {
        return RoomState::Bootstrap;
    }

    RoomState::Basic
}
