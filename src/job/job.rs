use std::fmt::Display;

use screeps::{ObjectId, Resource, Source};

#[derive(Clone, Copy, Debug)]
pub enum JobType {
    StaticMine(ObjectId<Source>),
    GetDroppedEnergy(ObjectId<Resource>),
}

impl Display for JobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::StaticMine(_) => write!(f, "StaticMine"),
            &Self::GetDroppedEnergy(_) => write!(f, "GetDroppedEnergy"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type: JobType,
}
