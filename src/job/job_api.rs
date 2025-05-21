use screeps::{
    find, Creep, HasId, HasPosition, ResourceType, Room, StructureProperties, StructureType,
};

use crate::job::job_utils::filter_has_store_space;

use super::job::{Job, JobType};

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

pub fn get_energy_job(_room: &Room, creep: &Creep) -> Option<Job> {
    // Look for only dropped energy right now (how miners operate early on)
    let dropped_energy_job = match creep
        .pos()
        .find_closest_by_path(find::DROPPED_RESOURCES, None)
        .filter(|resource| resource.resource_type() == ResourceType::Energy)
    {
        Some(energy) => Some(Job {
            job_type: JobType::GetDroppedEnergy(energy.id()),
        }),
        None => None,
    };

    dropped_energy_job
}

// This structure could probably use some cleaning up/functionizing
pub fn get_fill_structures_job(_room: &Room, creep: &Creep) -> Option<Job> {
    let my_extension = creep
        .pos()
        .find_closest_by_path(find::MY_STRUCTURES, None)
        .filter(|structure| {
            structure.structure_type() == StructureType::Extension
                && filter_has_store_space(structure)
        });

    let fill_extension_job = match my_extension {
        Some(extension) => Some(Job {
            job_type: JobType::FillStructure(extension.as_structure().id()),
        }),
        None => None,
    };

    fill_extension_job
}

pub fn get_fill_spawns_job(_room: &Room, creep: &Creep) -> Option<Job> {
    let my_spawn = creep
        .pos()
        .find_closest_by_path(find::MY_SPAWNS, None)
        .filter(|structure| {
            structure
                .store()
                .get_free_capacity(Some(ResourceType::Energy))
                > 0
        });

    let fill_spawn_job = match my_spawn {
        Some(spawn) => Some(Job {
            job_type: JobType::FillSpawn(spawn.id()),
        }),
        None => None,
    };

    fill_spawn_job
}
