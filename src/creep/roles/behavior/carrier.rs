use log::warn;
use screeps::{
    game, Creep, HasPosition, Resource, ResourceType, Room, SharedCreepProperties, Source,
    Structure, StructureController, StructureObject, Transferable, TransferableObject,
};

use crate::{
    creep::roles::roles_api::Roles,
    job::{
        job::{Job, JobType},
        job_api::{
            get_energy_job, get_fill_structures_job, get_mining_job, get_upgrade_controller_job,
        },
        job_utils::creep_set_job,
    },
    spawn::spawn_utils::get_living_creep_counts,
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

            // Fallback to mining your own energy if there are no miners
            let creep_counts = get_living_creep_counts(room);
            let miner_count = creep_counts.get(&Roles::Miner).unwrap_or(&0);
            if *miner_count == 0 {
                if let Some(mining_job) = get_mining_job(room, &self.creep) {
                    creep_set_job(&self.creep, mining_job);
                    return Some(mining_job);
                }
            }

            return None;
        }

        if let Some(fill_structures_job) = get_fill_structures_job(room, &self.creep) {
            creep_set_job(&self.creep, fill_structures_job);
            return Some(fill_structures_job);
        }

        if let Some(upgrade_controller_job) = get_upgrade_controller_job(room) {
            creep_set_job(&self.creep, upgrade_controller_job);
            return Some(upgrade_controller_job);
        }

        None
    }

    // TODO: Need a way to remove a finished job
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
            JobType::UpgradeController(controller_id) => {
                let controller =
                    game::get_object_by_id_typed::<StructureController>(&controller_id).unwrap();
                do_upgrade_controller_job(&self.creep, &controller);
            }
            JobType::SelfMining(source_id) => {
                let source = game::get_object_by_id_typed(&source_id).unwrap();
                do_self_mining_job(&self.creep, &source);
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

fn do_upgrade_controller_job(creep: &Creep, controller: &StructureController) {
    if creep.pos().in_range_to(controller.pos(), 3) {
        let _ = creep.upgrade_controller(controller);
    } else {
        let _ = creep.move_to(controller);
    }
}

fn do_self_mining_job(creep: &Creep, source: &Source) {
    if creep.pos().is_near_to(source.pos()) {
        let _ = creep.harvest(source);
    } else {
        let _ = creep.move_to(source);
    }
}

