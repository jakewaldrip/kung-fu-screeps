use screeps::{find, HasId, Room};

use super::job::{Job, JobType};

pub fn get_static_mining_job(room: &Room) -> Option<Job> {
    if let Some(source) = room.find(find::SOURCES_ACTIVE, None).first() {
        let job = Job {
            job_type: JobType::StaticMine(source.id()),
        };
        Some(job)
    } else {
        None
    }
}
