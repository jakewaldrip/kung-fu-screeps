use std::collections::HashMap;

use screeps::{game, Creep, Part, Room};

use crate::{creep::roles::roles_api::Roles, memory::creep_memory::CreepMemory};

pub fn get_living_creep_counts(room: &Room) -> HashMap<Roles, u8> {
    let mut creep_count: HashMap<Roles, u8> = HashMap::new();
    let room_name = room.name().to_string();
    for creep in game::creeps().values() {
        let creep_memory = CreepMemory::get(&creep);
        let home_room = creep_memory.home_room;
        let creep_role = creep_memory.role;

        if room_name == home_room {
            *creep_count.entry(creep_role.clone()).or_insert(0) += 1;
        }
    }

    creep_count
}

pub fn get_part_count_by_type(creep: &Creep, body_part: &Part) -> u32 {
    let creep_body = creep.body();
    creep_body
        .iter()
        .filter_map(|p| {
            if &p.part() == body_part {
                Some(p.part().cost())
            } else {
                None
            }
        })
        .sum()
}
