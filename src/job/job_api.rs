use screeps::{find, Creep, HasId, HasPosition, ResourceType, Room};

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
