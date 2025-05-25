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
//
//
// One tricky piece with the updating is how to carry that across ticks. You likely want to
// generate new jobs every X ticks, and you don't want a new job to be created a for a pile of
// energy that another creep is already targetting, as the data wouldn't be up to date anymore.
//
// The simple solution is to maintain jobs until they are completed, but the energy isn't always
// accurate since, in the example of a pile of energy, you could have a miner adding to it every
// tick. At what point do you decide to update the energy?
//
// One option when generating jobs is to check existing jobs for validity, and update their energy
// based on current targeting creeps. This could be simplified by giving jobs a uuid that could be
// referenced to get the accurate job amount when updating for this tick
//
// We also run into another issue with that, how do you then decide to create new jobs? For that
// pile of energy, we have an existing job, how do we know when creating new jobs that we are not
// duplicating that job, or creating far less jobs than we should be (ie, 3 piles of energy)?
//
// One option for that problem is to look at the ID of the object being referenced. We know that
// only one GetDroppedEnergy job can exist for a single pile of dropped energy, so we could simply
// check if that ID has an associated FillJob in the Vec already
//
//
// Another architecure related decision tree, do we store a Vec<Job> for the room, or do we create
// a struct that has various types of jobs in them? This could help in organization as well as
// performance to have separated Vecs that we can search for jobs rather than one large one. The
// more I type the more that sells itself to me
#[derive(Clone, Copy, Debug)]
pub enum JobType {
    StaticMine(ObjectId<Source>),
    SelfMining(ObjectId<Source>),
    GetDroppedEnergy(ObjectId<Resource>),
    FillStructure(ObjectId<Structure>),
    UpgradeController(ObjectId<StructureController>),
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type: JobType,
}
