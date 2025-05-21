use screeps::{Creep, ResourceType, SharedCreepProperties, StructureObject, StructureProperties, StructureType};

use crate::creep::roles::behavior::creep_behavior::CREEP_JOB;

use super::job::Job;

pub fn get_creeps_current_job(creep_name: &str) -> Option<Job> {
    CREEP_JOB.with(|creep_job_refcell| {
        let creep_jobs = creep_job_refcell.borrow_mut();
        creep_jobs.get(creep_name).cloned()
    })
}

pub fn creep_set_job(creep: &Creep, job: Job) {
    CREEP_JOB.with(|creep_job_refcell| {
        let mut creep_jobs = creep_job_refcell.borrow_mut();
        creep_jobs.insert(creep.name(), job)
    });
}

pub fn filter_has_store_space(structure: &StructureObject) -> bool {
    let store = structure.as_has_store().unwrap().store();
    store.get_free_capacity(Some(ResourceType::Energy)) > 0
}

pub fn filter_is_fill_structure(structure: &StructureObject) -> bool {
    match structure.structure_type() {
        StructureType::Extension => true,
        StructureType::Storage => true,
        _ => false
    }
}
