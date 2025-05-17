use log::warn;
use screeps::{game, RoomName, SharedCreepProperties};
use std::str::FromStr;

use crate::{
    creep::roles::roles_api::{get_creep_behavior_impl, Roles},
    job::job_utils::{creep_has_job, get_creeps_current_job},
    memory::creep_memory::CreepMemory,
};

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

        let job = if creep_has_job(&creep) {
            get_creeps_current_job(&creep.name())
        } else {
            creep_behavior_impl.get_job(&room)
        };

        match job {
            Some(job) => creep_behavior_impl.do_job(&room, &job),
            None => warn!("Idling creep: {}", creep.name()),
        }
    }
}
