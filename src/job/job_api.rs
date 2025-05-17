use screeps::{find, HasId, Room};

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
