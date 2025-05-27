use crate::{
    job::room_jobs::RoomJobs, memory::memory_api::get_owned_rooms, room::room_api::set_room_state,
};

use super::room_cache::RoomCache;

pub fn run_room_manager() {
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        set_room_state(&room);
        RoomJobs::create_for_room(&room);
        RoomCache::update_for_room(&room);
    }
}
