use screeps::{find, game, Creep, HasId, HasPosition, ResourceType, Room, StructureObject};

use crate::job::job_utils::filter_has_store_space;

use super::{
    job::{Job, JobType},
    job_utils::filter_is_fill_structure,
};

pub fn is_job_done(creep: &Creep, job: &Job) -> bool {
    match job.job_type {
        JobType::StaticMine(_) => false,
        JobType::GetDroppedEnergy(resource_id) => {
            let dropped_energy = game::get_object_by_id_typed(&resource_id).unwrap();
            dropped_energy.amount() == 0 || creep.store().get_free_capacity(None) == 0
        }
        JobType::FillStructure(structure_id) => {
            let structure = game::get_object_by_id_typed(&structure_id).unwrap();
            let store = StructureObject::from(structure)
                .as_has_store()
                .unwrap()
                .store();

            store.get_free_capacity(None) == 0 || creep.store().get_used_capacity(None) == 0
        }
        JobType::UpgradeController(_) => {
            creep.store().get_used_capacity(Some(ResourceType::Energy)) == 0
        }
        JobType::SelfMining(source_id) => {
            let source = game::get_object_by_id_typed(&source_id).unwrap();
            creep.store().get_free_capacity(Some(ResourceType::Energy)) == 0 || source.energy() == 0
        }
    }
}

pub fn get_static_mining_job(room: &Room) -> Option<Job> {
    let static_mining_job = match room.find(find::SOURCES_ACTIVE, None).first() {
        Some(source) => {
            let job = Job {
                job_type: JobType::StaticMine(source.id()),
            };
            Some(job)
        }
        None => None,
    };

    static_mining_job
}

pub fn get_mining_job(_room: &Room, creep: &Creep) -> Option<Job> {
    

    creep.pos().find_closest_by_path(find::SOURCES_ACTIVE, None).map(|source| Job {
            job_type: JobType::SelfMining(source.id()),
        })
}

pub fn get_energy_job(_room: &Room, creep: &Creep) -> Option<Job> {
    // Look for only dropped energy right now (how miners operate early on)
    

    creep
        .pos()
        .find_closest_by_path(find::DROPPED_RESOURCES, None)
        .filter(|resource| resource.resource_type() == ResourceType::Energy).map(|energy| Job {
            job_type: JobType::GetDroppedEnergy(energy.id()),
        })
}

pub fn get_fill_structures_job(_room: &Room, creep: &Creep) -> Option<Job> {
    let fill_structure = creep
        .pos()
        .find_closest_by_path(find::MY_STRUCTURES, None)
        .filter(|structure| {
            filter_is_fill_structure(structure) && filter_has_store_space(structure)
        });

    if let Some(fill_structure) = fill_structure {
        return Some(Job {
            job_type: JobType::FillStructure(fill_structure.as_structure().id()),
        });
    }

    let my_spawn = creep
        .pos()
        .find_closest_by_path(find::MY_SPAWNS, None)
        .filter(|structure| {
            structure
                .store()
                .get_free_capacity(Some(ResourceType::Energy))
                > 0
        });

    if let Some(spawn) = my_spawn {
        let spawn_structure = StructureObject::from(spawn);
        return Some(Job {
            job_type: JobType::FillStructure(spawn_structure.as_structure().id()),
        });
    }

    None
}

pub fn get_upgrade_controller_job(room: &Room) -> Option<Job> {
    

    room.controller().map(|controller| Job {
            job_type: JobType::UpgradeController(controller.id()),
        })
}
