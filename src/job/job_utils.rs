use screeps::{Creep, SharedCreepProperties};

use crate::creep::roles::behavior::creep_behavior::CREEP_JOB;

use super::job::Job;

pub fn get_creeps_current_job(creep_name: &str) -> Option<Job> {
    CREEP_JOB.with(|creep_job_refcell| {
        let creep_jobs = creep_job_refcell.borrow_mut();
        creep_jobs.get(creep_name).cloned()
    })
}

pub fn creep_has_job(creep: &Creep) -> bool {
    CREEP_JOB.with(|creep_job_refcell| {
        let creep_jobs = creep_job_refcell.borrow();
        let creep_job = creep_jobs.get(&creep.name().to_string());
        creep_job.is_some()
    })
}

pub fn creep_set_job(creep: &Creep, job: Job) {
    CREEP_JOB.with(|creep_job_refcell| {
        let mut creep_jobs = creep_job_refcell.borrow_mut();
        creep_jobs.insert(creep.name(), job)
    });
}
