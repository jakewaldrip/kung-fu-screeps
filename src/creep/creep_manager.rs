use screeps::{game, RoomName};
use std::str::FromStr;

use crate::{
    creep::roles::roles_api::{get_creep_behavior_impl, Roles},
    memory::creep_memory::CreepMemory,
};

use super::creep_api;

pub fn run_creep_manager() {
    for creep in game::creeps().values() {
        if creep.spawning() {
            return ();
        }

        let creep_memory = CreepMemory::get(&creep);
        let role: Roles = creep_memory.role;
        let creep_behavior_impl = get_creep_behavior_impl(&role, creep.clone()).unwrap();

        let room_name = RoomName::from_str(&creep_memory.home_room).unwrap();
        let room = game::rooms().get(room_name).unwrap();

        if creep_api::creep_has_job(&creep) {
            creep_behavior_impl.do_job(&room);
        } else {
            creep_behavior_impl.get_job(&room);
        }
    }
}
