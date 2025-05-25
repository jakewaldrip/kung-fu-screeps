use std::{cell::RefCell, collections::HashMap};

use screeps::{Creep, Room, RoomName};

use crate::memory::memory_api::get_creeps_in_room;

use super::job::Job;

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
fn create_static_mining_jobs(_room: &Room, _creeps: &Vec<Creep>) -> Vec<Job> {
    todo!()
}

fn create_get_energy_jobs(_room: &Room, _creeps: &Vec<Creep>) -> Vec<Job> {
    todo!()
}

fn create_fill_structure_jobs(_room: &Room, _creeps: &Vec<Creep>) -> Vec<Job> {
    todo!()
}

fn create_upgrade_jobs(_room: &Room, _creeps: &Vec<Creep>) -> Vec<Job> {
    todo!()
}
