use log::warn;
use screeps::{
    game, Creep, HasPosition, Resource, ResourceType, Room, SharedCreepProperties, Structure,
    StructureObject, StructureSpawn, Transferable, TransferableObject,
};

use crate::job::{
    job::{Job, JobType},
    job_api::{get_energy_job, get_fill_spawns_job, get_fill_structures_job},
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

        // Get energy if we're out first, early return if we can't
        if used_capacity == 0 {
            if let Some(energy_job) = get_energy_job(room, &self.creep) {
                creep_set_job(&self.creep, energy_job);
                return Some(energy_job);
            }
            return None;
        }

        if let Some(fill_extensions_job) = get_fill_structures_job(room, &self.creep) {
            creep_set_job(&self.creep, fill_extensions_job);
            return Some(fill_extensions_job);
        }

        if let Some(fill_spawn_job) = get_fill_spawns_job(room, &self.creep) {
            creep_set_job(&self.creep, fill_spawn_job);
            return Some(fill_spawn_job);
        }

        None
    }

    fn do_job(&self, _room: &Room, job: &Job) -> () {
        match job.job_type {
            JobType::GetDroppedEnergy(resource_id) => {
                let resource = game::get_object_by_id_typed::<Resource>(&resource_id).unwrap();
                do_pickup_dropped_energy_job(&self.creep, &resource);
            }
            JobType::FillStructure(structure_id) => {
                let structure = game::get_object_by_id_typed::<Structure>(&structure_id).unwrap();
                let transferable_structure =
                    TransferableObject::try_from(StructureObject::from(structure));
                do_fill_structure_job(&self.creep, &transferable_structure.unwrap());
            }
            JobType::FillSpawn(spawn_id) => {
                let spawn = game::get_object_by_id_typed::<StructureSpawn>(&spawn_id).unwrap();
                do_fill_structure_job(&self.creep, &spawn);
            }
            _ => warn!(
                "{} obtained unhandled job type: {:?}",
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

fn do_fill_structure_job<T: Transferable>(creep: &Creep, structure: &T) {
    if creep.pos().is_near_to(structure.pos()) {
        let _ = creep.transfer(structure, ResourceType::Energy, None);
    } else {
        let _ = creep.move_to(structure);
    }
}
