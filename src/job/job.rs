use screeps::{ObjectId, Source};

pub enum JobType {
    StaticMine(ObjectId<Source>),
}

pub struct Job {
    pub job_type: JobType,
}
