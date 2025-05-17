use log::warn;
use screeps::{game, Creep, HasPosition, Room, Source};

use crate::job::{
    job::{Job, JobType},
    job_api::get_static_mining_job,
    job_utils::creep_set_job,
};

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
    fn get_job(&self, room: &Room) -> Option<Job> {
        let static_mining_job = get_static_mining_job(room);
        match static_mining_job {
            Some(job) => {
                creep_set_job(&self.creep, job);
            }
            None => warn!("Miner couldn't find static mining job"),
        }
        static_mining_job
    }

    fn do_job(&self, _room: &Room, job: &Job) -> () {
        match job.job_type {
            JobType::StaticMine(source_id) => {
                let source = game::get_object_by_id_typed::<Source>(&source_id).unwrap();
                do_static_mine_job(&self.creep, source);
            }
        }
    }
}

fn do_static_mine_job(creep: &Creep, source: Source) {
    if creep.pos().is_near_to(source.pos()) {
        let _ = creep.harvest(&source);
    } else {
        let _ = creep.move_to(source);
    }
}
