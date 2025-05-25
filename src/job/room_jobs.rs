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

impl RoomJobs {
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
        })
    }
}

// TODO: Complete these
fn create_static_mining_jobs(_room: &Room, _creeps: &[Creep]) -> Vec<Job> {
    // Get sources for room
    // Introduce room level memory cache here as well
    // Basic concept: room_memory_api::get_sources(room) -> Vec<Source>
    // It will store IDs, but it will return objects, different bucket for different objects
    // sources: RefCell<Vec<ObjectId<Source>>>
    // structures: RefCel<HashMap<StructureType, ObjectId<Structure>>>
    //
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
