use log::warn;
use screeps::{game, BodyPart, Creep, Part, ResourceType, Room, StructureObject};

use super::{room_jobs::{RoomJobTypes, RoomJobs}, Job, JobType};

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

// TODO: complete
pub fn get_static_mining_job(room: &Room, creep: &Creep) -> Option<Job> {
    let filter_fn = |job: &mut Job| {
        match job.job_type {
            JobType::StaticMine(static_mine_data) => {
                static_mine_data.work_parts_remaining > 0
            },
            _ => {
                warn!("Job type and room job mismatch");
                false
            }
        }
    };

    let update_fn = |job: &mut Job| {
        let binding = creep.body();
        let work_parts: Vec<&BodyPart> = binding.iter().filter(|p| p.part() == Part::Work).collect();
        match job.job_type {
             JobType::StaticMine(mut static_mine_data) => {
                static_mine_data.work_parts_remaining = static_mine_data.work_parts_remaining.saturating_sub(work_parts.len() as u32);
            },
            _ => {
                warn!("Job type and room job mismatch");
            }
        }
    };

    RoomJobs::get_job_and_update(
        room, 
        RoomJobTypes::StaticMining, 
        filter_fn, 
        Some(update_fn)
    )
}

// TODO: complete
pub fn get_mining_job(_room: &Room, _creep: &Creep) -> Option<Job> {
    todo!()
}

// TODO: complete
pub fn get_energy_job(_room: &Room, _creep: &Creep) -> Option<Job> {
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
