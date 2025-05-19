use screeps::{Creep, Room};

use crate::job::job::{Job, JobType};

use super::creep_behavior::CreepBehavior;

pub struct CarrierBehavior {
    creep: Creep,
}

impl CarrierBehavior {
    pub fn get(creep: Creep) -> Self {
        Self { creep }
    }
}

impl CreepBehavior for CarrierBehavior {
    fn get_job(&self, _room: &Room) -> Option<Job> {
        todo!()
    }

    // TODO
    fn do_job(&self, _room: &Room, job: &Job) -> () {
        match job.job_type {
            JobType::StaticMine(_source_id) => {
                todo!()
            }
        }
    }
}
