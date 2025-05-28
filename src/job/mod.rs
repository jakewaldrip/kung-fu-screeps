pub mod job_api;
pub mod job_utils;
pub mod room_jobs;

use screeps::{ObjectId, Resource, Source, Structure, StructureController};

// INFO: Idea here is to collect jobs in the room so we can track who is taking what
// Jobs will be stored in the ROOM_JOBS hashmap for each room, and creeps will select from here
//
// We need to find a place to store memory on the job, likely within the jobtype enum on structs
// This memory will include things such as energy remaining, which will be updated by the creep
// when they set their job. This will allow the next creep to consider that when picking a job

#[derive(Clone, Copy, Debug)]
pub enum JobType {
    SelfMining(ObjectId<Source>),
    UpgradeController(ObjectId<StructureController>),

    StaticMine(StaticMineData),
    GetDroppedEnergy(GetDroppedEnergyData),
    FillStructure(FillStructureData),
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type: JobType,
}

#[derive(Clone, Copy, Debug)]
pub struct StaticMineData {
    pub source_id: ObjectId<Source>,
    pub work_parts_remaining: u32,
}

impl StaticMineData {
    pub fn new_from_data(source_id: &ObjectId<Source>, work_parts_remaining: u32) -> Self {
        StaticMineData {
            source_id: *source_id,
            work_parts_remaining,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GetDroppedEnergyData {
    pub resource_id: ObjectId<Resource>,
    pub energy_remaining: u32,
}

impl GetDroppedEnergyData {
    pub fn new_from_data(resource_id: &ObjectId<Resource>, energy_remaining: u32) -> Self {
        GetDroppedEnergyData {
            resource_id: *resource_id,
            energy_remaining,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FillStructureData {
    pub structure_id: ObjectId<Structure>,
    pub capacity_remaining: u32,
}

impl FillStructureData {
    pub fn new_from_data(structure_id: ObjectId<Structure>, capacity_remaining: u32) -> Self {
        FillStructureData {
            structure_id,
            capacity_remaining,
        }
    }
}
