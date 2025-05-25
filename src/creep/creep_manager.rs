use log::warn;
use screeps::{game, RoomName, SharedCreepProperties};
use std::str::FromStr;

use crate::{
    creep::roles::roles_api::{get_creep_behavior_impl, Roles},
    job::{job_api::is_job_done, job_utils::get_creeps_current_job},
    memory::creep_memory::CreepMemory,
};

pub fn run_creep_manager() {
    for creep in game::creeps().values() {
        if creep.spawning() {
            return ;
        }

        let creep_memory = CreepMemory::get(&creep);
        let role: Roles = creep_memory.role;
        let creep_behavior_impl = get_creep_behavior_impl(&role, creep.clone()).unwrap();

        let room_name = RoomName::from_str(&creep_memory.home_room).unwrap();
        let room = game::rooms().get(room_name).unwrap();

        let job = get_creeps_current_job(&creep.name())
            .and_then(|job| {
                if is_job_done(&creep, &job) {
                    creep_behavior_impl.get_job(&room)
                } else {
                    Some(job)
                }
            })
            .or_else(|| creep_behavior_impl.get_job(&room));

        match job {
            Some(job) => creep_behavior_impl.do_job(&room, &job),
            None => warn!("Idling creep: {}", creep.name()),
        }
    }
}
