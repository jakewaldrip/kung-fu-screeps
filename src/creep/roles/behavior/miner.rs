use screeps::{Creep, Room};

use crate::{creep::creep_api::creep_set_job, job::job_api::get_static_mining_job};

use super::creep_behavior::CreepBehavior;

pub struct MinerBehavior {
    creep: Creep,
}

impl MinerBehavior {
    pub fn get(creep: Creep) -> Self {
        Self { creep }
    }
}

impl CreepBehavior for MinerBehavior {
    fn get_job(&self, room: &Room) -> () {
        let static_mining_job = get_static_mining_job(room);
        match static_mining_job {
            Some(job) => creep_set_job(&self.creep, job),
            None => (),
        }
    }

    fn do_job(&self, _room: &Room) -> () {
        todo!()
    }
}
