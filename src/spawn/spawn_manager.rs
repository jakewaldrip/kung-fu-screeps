use log::{info, warn};
use screeps::{game, Room, SpawnOptions};

use crate::{
    creep::roles::roles_api::get_creep_data_impl, memory::memory_api::get_owned_rooms,
    spawn::spawn_api::get_next_role_to_spawn,
};

pub fn run_spawn_manager() {
    let owned_rooms = get_owned_rooms();
    for room in owned_rooms {
        run_spawns_for_room(room);
    }
}

pub fn run_spawns_for_room(room: Room) {
    let room_name = room.name().to_string();
    let spawn_for_room = game::spawns().get("Spawn1".into()).unwrap();

    if let Some(next_role_to_spawn) = get_next_role_to_spawn(&room) {
        let creep_data_impl = get_creep_data_impl(&next_role_to_spawn).unwrap();
        let creep_body = creep_data_impl.get_body(&room);
        let body_cost = creep_body.iter().map(|p| p.cost()).sum();

        if room.energy_available() >= body_cost {
            let creep_name = creep_data_impl.get_name(room_name.clone());
            let creep_memory = creep_data_impl.get_memory(room_name.clone());
            let spawn_options = SpawnOptions::new().memory(creep_memory.to_js());

            let spawn_result =
                spawn_for_room.spawn_creep_with_options(&creep_body, &creep_name, &spawn_options);
            match spawn_result {
                Ok(_) => info!("Spawned new creep"),
                Err(err) => warn!("Failed to spawn {}: {}", next_role_to_spawn, err),
            }
        }
    }
}
