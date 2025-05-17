use log::{info, warn};
use screeps::{Room, SpawnOptions};

use crate::{
    creep::roles::roles_api::get_creep_data_impl, memory::memory_api::get_owned_rooms,
    spawn::spawn_api::get_next_role_to_spawn,
};

use super::spawn_api::get_active_spawn_for_room;

pub fn run_spawn_manager() {
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        run_spawns_for_room(room);
    }
}

pub fn run_spawns_for_room(room: Room) {
    let room_name = room.name().to_string();
    let active_spawn_for_room = match get_active_spawn_for_room(&room) {
        Some(spawn) => spawn,
        None => return,
    };
    let next_role_to_spawn = match get_next_role_to_spawn(&room) {
        Some(role) => role,
        None => return,
    };
    let creep_data_impl = match get_creep_data_impl(&next_role_to_spawn) {
        Some(creep_data_impl) => creep_data_impl,
        None => {
            warn!("No implementation found for {}", next_role_to_spawn);
            return;
        }
    };

    let creep_body = creep_data_impl.get_body(&room);
    let body_cost = creep_body.iter().map(|p| p.cost()).sum();

    if room.energy_available() >= body_cost {
        let creep_name = creep_data_impl.get_name(room_name.clone());
        let creep_memory = creep_data_impl.get_memory(room_name.clone());
        let spawn_options = SpawnOptions::new().memory(creep_memory.to_js());

        let spawn_result = active_spawn_for_room.spawn_creep_with_options(
            &creep_body,
            &creep_name,
            &spawn_options,
        );
        match spawn_result {
            Ok(_) => info!("Spawned new creep: {}", creep_name),
            Err(err) => warn!("Failed to spawn {}: {}", next_role_to_spawn, err),
        }
    }
}
