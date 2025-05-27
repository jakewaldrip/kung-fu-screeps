use std::{cell::RefCell, collections::HashMap};

use screeps::{Creep, Room, RoomName};

use crate::memory::memory_api::get_creeps_in_room;

use super::Job;

thread_local! {
    pub static ROOM_JOBS: RefCell<HashMap<RoomName, RoomJobs>> = RefCell::new(HashMap::new());
}

pub struct RoomJobs {
    static_mining_jobs: Vec<Job>,
    get_energy_jobs: Vec<Job>,
    fill_structure_jobs: Vec<Job>,
    upgrade_jobs: Vec<Job>,
}

pub enum RoomJobTypes {
    StaticMining,
    GetEnergy,
    FillStructure,
    Upgrade,
}

impl RoomJobs {
    /// Creates jobs for a room, separated into vectors for each job type
    pub fn create_for_room(room: &Room) {
        let creeps = get_creeps_in_room(room);
        let room_name = room.name();
        let room_jobs = RoomJobs {
            static_mining_jobs: create_static_mining_jobs(room, &creeps),
            get_energy_jobs: create_get_energy_jobs(room, &creeps),
            fill_structure_jobs: create_fill_structure_jobs(room, &creeps),
            upgrade_jobs: create_upgrade_jobs(room, &creeps),
        };

        ROOM_JOBS.with(|room_jobs_ref| {
            let mut room_jobs_memory = room_jobs_ref.borrow_mut();
            room_jobs_memory.insert(room_name, room_jobs);
        });
    }

    /// Gets a job from the room_job cache and updates its memory in the heap
    /// room: The room we are seeking a job in
    /// job_type - the type of job we are looking for. Needed to narrow down to a precise vec of jobs
    /// filter_fn - Filter to apply to the job vec
    /// update_fn_option - Optional update fn to apply to the job data once a creep has grabbed it.
    /// For example, subtracting the creep's carry capacity from a containers remaining energy
    pub fn _get_job_and_update<F, U>(
        room: &Room,
        job_type: RoomJobTypes,
        filter_fn: F,
        update_fn_option: Option<U>,
    ) -> Option<Job>
    where
        F: Fn(&mut Job) -> bool,
        U: FnOnce(&mut Job),
    {
        ROOM_JOBS.with(|room_jobs_ref| {
            let mut room_jobs_memory = room_jobs_ref.borrow_mut();
            if let Some(room_jobs) = room_jobs_memory.get_mut(&room.name()) {
                let jobs_of_type = match job_type {
                    RoomJobTypes::StaticMining => &mut room_jobs.static_mining_jobs,
                    RoomJobTypes::GetEnergy => &mut room_jobs.get_energy_jobs,
                    RoomJobTypes::FillStructure => &mut room_jobs.fill_structure_jobs,
                    RoomJobTypes::Upgrade => &mut room_jobs.upgrade_jobs,
                };

                // TODO: Consider .find parameter to select a job more precisely
                // All filtered jobs are considered valid
                let mut valid_jobs: Vec<&mut Job> = Vec::new();
                for job in jobs_of_type.iter_mut() {
                    if filter_fn(job) {
                        valid_jobs.push(job);
                    }
                }

                // swap_remove will panic if vec is empty
                if valid_jobs.is_empty() {
                    return None;
                }

                let creep_job = valid_jobs.swap_remove(0);
                if let Some(update_fn) = update_fn_option {
                    update_fn(creep_job);
                }
                return Some(creep_job.clone());
            }

            return None;
        });

        None
    }
}

// TODO: Complete these
fn create_static_mining_jobs(_room: &Room, _creeps: &[Creep]) -> Vec<Job> {
    // Get sources for room
    // Create a job for each
    // Introduce concept of job level memory here, in this case work_parts_remaining: 5
    //
    // For each source, before creating the job get the number of creeps that currently have it and adjust accordingly
    // We can find that by looking for static mining jobs that target the same source
    // Subtract their work parts from work_parts_remaining, and if > 0 create the job with that number
    todo!()
}

fn create_get_energy_jobs(_room: &Room, _creeps: &[Creep]) -> Vec<Job> {
    todo!()
}

fn create_fill_structure_jobs(_room: &Room, _creeps: &[Creep]) -> Vec<Job> {
    todo!()
}

fn create_upgrade_jobs(_room: &Room, _creeps: &[Creep]) -> Vec<Job> {
    todo!()
}
