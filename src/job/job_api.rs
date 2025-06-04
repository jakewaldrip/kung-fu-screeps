use log::warn;
use screeps::{game, BodyPart, Creep, HasPosition, Part, ResourceType, Room, StructureObject};

use super::{room_jobs::ROOM_JOBS, Job, JobType};

pub fn is_job_done(creep: &Creep, job: &Job) -> bool {
    match job.job_type {
        JobType::StaticMine(_) => false,
        JobType::GetDroppedEnergy(job_data) => {
            let dropped_energy = game::get_object_by_id_typed(&job_data.resource_id).unwrap();
            dropped_energy.amount() == 0 || creep.store().get_free_capacity(None) == 0
        }
        JobType::FillStructure(job_data) => {
            let structure = game::get_object_by_id_typed(&job_data.structure_id).unwrap();
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

pub fn get_static_mining_job(room: &Room, creep: &Creep) -> Option<Job> {
    let update_fn = |job: &mut Job| {
        let work_parts = creep
            .body()
            .iter()
            .filter(|p| p.part() == Part::Work)
            .collect::<Vec<&BodyPart>>()
            .len() as u32;

        if let Some(static_mine_data) = job.job_type.as_mut_static_mine() {
            static_mine_data.work_parts_remaining = static_mine_data
                .work_parts_remaining
                .saturating_sub(work_parts);
        }
    };

    ROOM_JOBS.with_borrow_mut(|room_jobs_memory| {
        let room_jobs = room_jobs_memory.get_mut(&room.name()).or_else(|| {
            warn!("Jobs not found for room: {}", room.name());
            None
        })?;

        let jobs_of_type = &mut room_jobs.static_mining_jobs;

        // TODO: consider open squares in the filter
        let mut valid_jobs: Vec<&mut Job> = jobs_of_type
            .iter_mut()
            .filter(|job| {
                if let Some(static_mine_data) = job.job_type.as_static_mine() {
                    static_mine_data.work_parts_remaining > 0
                } else {
                    false
                }
            })
            .collect();

        // swap_remove will panic if vec is empty
        if valid_jobs.is_empty() {
            return None;
        }

        // find index of closest of the creep jobs to choose
        let (creep_job_index, _) = valid_jobs
            .iter()
            .enumerate()
            .min_by_key(|(_, job)| {
                if let Some(static_mine_data) = job.job_type.as_static_mine() {
                    let source = game::get_object_by_id_typed(&static_mine_data.source_id).unwrap();
                    creep.pos().get_range_to(source.pos())
                } else {
                    unreachable!()
                }
            })
            .unwrap();

        let creep_job = valid_jobs.swap_remove(creep_job_index);
        update_fn(creep_job);

        // returns copy of the job, ownership cannot leave scope
        Some(*creep_job)
    })
}

pub fn get_mining_job(room: &Room, creep: &Creep) -> Option<Job> {
    ROOM_JOBS.with_borrow_mut(|room_jobs_memory| {
        let room_jobs = room_jobs_memory.get_mut(&room.name()).or_else(|| {
            warn!("Jobs not found for room: {}", room.name());
            None
        })?;

        let jobs_of_type = &mut room_jobs.get_energy_jobs;

        // TODO: calculate open squares and creeps assigned, consider that in the filter
        let valid_jobs: Vec<&Job> = jobs_of_type
            .iter()
            .filter(|job| job.job_type.as_self_mining().is_some())
            .collect();

        // swap_remove will panic if vec is empty
        if valid_jobs.is_empty() {
            return None;
        }

        // find closest job to the creep
        let creep_job = valid_jobs
            .iter()
            .min_by_key(|job| {
                if let Some(source_id) = job.job_type.as_self_mining() {
                    let source = game::get_object_by_id_typed(source_id).unwrap();
                    creep.pos().get_range_to(source.pos())
                } else {
                    unreachable!()
                }
            })
            .unwrap();

        Some(**creep_job)
    })
}

// TODO: complete
pub fn get_energy_from_structure_job(_room: &Room, _creep: &Creep) -> Option<Job> {
    todo!()
}

// TODO: complete
pub fn get_fill_structures_job(_room: &Room, _creep: &Creep) -> Option<Job> {
    todo!()
}

// TODO: complete
pub fn get_upgrade_controller_job(_room: &Room) -> Option<Job> {
    todo!()
}
