use crate::{memory::memory_api::get_owned_rooms, room::room_api::set_room_state};

pub fn run_room_manager() {
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        set_room_state(room);
    }
}
