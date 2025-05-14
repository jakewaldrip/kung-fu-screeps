use super::creep_data::CreepData;

pub struct MinerData {}

impl MinerData {
    pub fn get() -> Self {
        Self {}
    }
}

impl CreepData for MinerData {
    fn get_name(&self) -> String {
        "Jimmy".into()
    }
}
