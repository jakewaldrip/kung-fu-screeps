use screeps::{Creep, SharedCreepProperties};

use crate::job::job::Job;

use super::roles::behavior::creep_behavior::CREEP_JOB;

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
        creep_jobs.insert(creep.name(), job).unwrap()
    });
}
