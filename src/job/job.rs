use screeps::{ObjectId, Resource, Source, Structure, StructureController};

#[derive(Clone, Copy, Debug)]
pub enum JobType {
    StaticMine(ObjectId<Source>),
    GetDroppedEnergy(ObjectId<Resource>),
    FillStructure(ObjectId<Structure>),
    UpgradeController(ObjectId<StructureController>),
    SelfMining(ObjectId<Source>),
}

#[derive(Clone, Copy, Debug)]
pub struct Job {
    pub job_type: JobType,
}
