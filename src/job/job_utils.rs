use screeps::{
    Creep, ObjectId, Part, ResourceType, SharedCreepProperties, Source, StructureObject,
    StructureProperties, StructureType,
};

use crate::{
    config::constants::WORK_PARTS_PER_SOURCE, creep::roles::behavior::creep_behavior::CREEP_JOB,
    spawn::spawn_utils::get_part_count_by_type,
};

use super::{Job, JobType};

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

pub fn _has_store_space(structure: &StructureObject) -> bool {
    let store = structure.as_has_store().unwrap().store();
    store.get_free_capacity(Some(ResourceType::Energy)) > 0
}

pub fn _is_fill_structure(structure: &StructureObject) -> bool {
    matches!(
        structure.structure_type(),
        StructureType::Extension | StructureType::Storage
    )
}

pub fn get_work_parts_assigned_to_source(creeps: &[Creep], source_id: &ObjectId<Source>) -> u32 {
    let mut work_parts_assigned = 0;
    for creep in creeps {
        match get_creeps_current_job(&creep.name()) {
            Some(creep_job) => {
                if let JobType::StaticMine(job_data) = creep_job.job_type {
                    if &job_data.source_id == source_id {
                        work_parts_assigned += get_part_count_by_type(creep, &Part::Work);
                    }
                }
            }
            None => continue,
        }
    }

    work_parts_assigned.max(WORK_PARTS_PER_SOURCE)
}

// TODO: complete this
pub fn get_carry_capacity_assigned_to_object<T>(creeps: &[Creep], object_id: &ObjectId<T>) -> u32 {
    todo!()
}
