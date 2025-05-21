use log::warn;
use screeps::{game, Creep, HasPosition, Resource, ResourceType, Room, SharedCreepProperties};

use crate::job::{
    job::{Job, JobType},
    job_api::get_energy_job,
    job_utils::creep_set_job,
};

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
    fn get_job(&self, room: &Room) -> Option<Job> {
        let used_capacity = self
            .creep
            .store()
            .get_used_capacity(Some(ResourceType::Energy));

        if used_capacity == 0 {
            if let Some(energy_job) = get_energy_job(room, &self.creep) {
                creep_set_job(&self.creep, energy_job);
                return Some(energy_job);
            }
            return None;
        }

        None
    }

    fn do_job(&self, _room: &Room, job: &Job) -> () {
        match job.job_type {
            JobType::GetDroppedEnergy(resource_id) => {
                let resource = game::get_object_by_id_typed::<Resource>(&resource_id).unwrap();
                do_pickup_dropped_energy_job(&self.creep, &resource);
            }
            _ => warn!(
                "{} obtained unhandled job type: {}",
                self.creep.name(),
                job.job_type
            ),
        }
    }
}

fn do_pickup_dropped_energy_job(creep: &Creep, resource: &Resource) {
    if creep.pos().is_near_to(resource.pos()) {
        let _ = creep.pickup(resource);
    } else {
        let _ = creep.move_to(resource);
    }
}
