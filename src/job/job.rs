use screeps::{ObjectId, Source};

#[derive(Clone, Copy, Debug)]
pub enum JobType {
    StaticMine(ObjectId<Source>),
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type: JobType,
}
