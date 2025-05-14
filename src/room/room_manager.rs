use log::info;
use screeps::game;

use crate::room::room_api::set_room_state;

pub fn run_room_manager() {
    info!("Running room manager");
    for room in game::rooms().values() {
        info!("Running room {}", room.name());
        set_room_state(room);
    }
}
