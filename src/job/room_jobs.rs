use std::{cell::RefCell, collections::HashMap};

use log::warn;
use screeps::{game, Creep, HasId, ObjectId, ResourceType, Room, RoomName, Structure};

use crate::{
    config::constants::WORK_PARTS_PER_SOURCE, memory::memory_api::get_creeps_in_room,
    room::room_cache::ROOM_CACHE,
};

use super::{
    job_utils::{get_carry_capacity_assigned_to_object, get_work_parts_assigned_to_source},
    FillStructureData, GetDroppedEnergyData, Job, JobType, StaticMineData,
};

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
        ROOM_JOBS.with_borrow_mut(|room_jobs_memory| {
            let room_jobs = match room_jobs_memory.get_mut(&room.name()) {
                Some(room_jobs) => room_jobs,
                None => {
                    warn!("Jobs not found for room: {}", room.name());
                    return None;
                }
            };

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

            // returns copy of the job, ownership cannot leave scope
            Some(*creep_job)
        })
    }
}

fn create_static_mining_jobs(room: &Room, creeps: &[Creep]) -> Vec<Job> {
    ROOM_CACHE.with_borrow(|room_cache_map| {
        let room_cache = match room_cache_map.get(&room.name()) {
            Some(room_cache) => room_cache,
            None => {
                warn!("Cache not found for room: {}", room.name());
                return Vec::new();
            }
        };

        let mut source_jobs: Vec<Job> = Vec::new();

        for source_id in &room_cache.sources {
            let work_parts_currently_assigned =
                get_work_parts_assigned_to_source(creeps, source_id);
            let work_parts_remaining = WORK_PARTS_PER_SOURCE - work_parts_currently_assigned;

            let job_type = JobType::StaticMine(StaticMineData::new_from_data(
                source_id,
                work_parts_remaining,
            ));
            source_jobs.push(Job { job_type });
        }

        source_jobs
    })
}

// TODO: Add more sources of energy
fn create_get_energy_jobs(room: &Room, creeps: &[Creep]) -> Vec<Job> {
    ROOM_CACHE.with_borrow(|room_cache_map| {
        let room_cache = match room_cache_map.get(&room.name()) {
            Some(room_cache) => room_cache,
            None => {
                warn!("Cache not found for room: {}", room.name());
                return Vec::new();
            }
        };

        let mut get_energy_jobs: Vec<Job> = Vec::new();

        // Dropped Energy
        for resource_id in &room_cache.dropped_resources {
            let resource = game::get_object_by_id_typed(resource_id).unwrap();
            if resource.resource_type() != ResourceType::Energy {
                continue;
            }

            let carry_capacity_currently_assigned =
                get_carry_capacity_assigned_to_object(creeps, resource_id);

            let energy_remaining = resource
                .amount()
                .saturating_sub(carry_capacity_currently_assigned);

            let job_type = JobType::GetDroppedEnergy(GetDroppedEnergyData::new_from_data(
                resource_id,
                energy_remaining,
            ));
            get_energy_jobs.push(Job { job_type });
        }

        // Self mining
        for source_id in &room_cache.sources {
            let source = game::get_object_by_id_typed(source_id).unwrap();
            // TODO: Smarter targeting for sources about to expire and currently being worked
            if source.energy() > 0 {
                let job_type = JobType::SelfMining(*source_id);
                get_energy_jobs.push(Job { job_type });
            }
        }

        get_energy_jobs
    })
}

// TODO: Add more structures that can be filled
fn create_fill_structure_jobs(room: &Room, creeps: &[Creep]) -> Vec<Job> {
    ROOM_CACHE.with_borrow(|room_cache_map| {
        let room_cache = match room_cache_map.get(&room.name()) {
            Some(room_cache) => room_cache,
            None => {
                warn!("Cache not found for room: {}", room.name());
                return Vec::new();
            }
        };

        let mut fill_structure_jobs: Vec<Job> = Vec::new();

        // TODO: Zip extensions into a slice to loop over here, mapping the objectId objects into
        // structure ID objects in that
        for spawn_id in &room_cache.spawns {
            let carry_capacity_currently_assigned =
                get_carry_capacity_assigned_to_object(creeps, spawn_id);
            let spawn = game::get_object_by_id_typed(spawn_id).unwrap();
            let free_capacity = spawn.store().get_free_capacity(Some(ResourceType::Energy));
            let capacity_remaining =
                (free_capacity - carry_capacity_currently_assigned as i32).max(0) as u32;

            // Unchecked conversion, safe in this case going from spawn -> structure
            // Need to error check/log these when used in general
            let structure_id = ObjectId::into_type::<Structure>(*spawn_id);

            let job_type = JobType::FillStructure(FillStructureData::new_from_data(
                structure_id,
                capacity_remaining,
            ));
            fill_structure_jobs.push(Job { job_type });
        }

        fill_structure_jobs
    })
}

fn create_upgrade_jobs(room: &Room, _creeps: &[Creep]) -> Vec<Job> {
    let controller = match room.controller() {
        Some(controller) => controller,
        None => {
            warn!("Controller not found in room: {}", room.name());
            return Vec::new();
        }
    };

    let job_type = JobType::UpgradeController(controller.id());
    vec![Job { job_type }]
}
