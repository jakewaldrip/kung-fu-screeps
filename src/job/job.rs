use screeps::{ObjectId, Resource, Source, Structure, StructureSpawn};

#[derive(Clone, Copy, Debug)]
pub enum JobType {
    StaticMine(ObjectId<Source>),
    GetDroppedEnergy(ObjectId<Resource>),
    FillStructure(ObjectId<Structure>),
    FillSpawn(ObjectId<StructureSpawn>),
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type: JobType,
}
