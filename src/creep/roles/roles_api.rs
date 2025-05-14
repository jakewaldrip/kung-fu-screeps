use screeps::Creep;

use crate::creep::roles::{behavior::miner::MinerBehavior, data::miner::MinerData};

use super::{behavior::creep_behavior::CreepBehavior, data::creep_data::CreepData};

pub enum Roles {
    Miner,
}

pub fn get_creep_data_impl(role: &Roles) -> Option<Box<dyn CreepData>> {
    match role {
        Roles::Miner => Some(Box::new(MinerData::get())),
    }
}

pub fn get_creep_behavior_impl(role: &Roles, creep: Creep) -> Option<Box<dyn CreepBehavior>> {
    match role {
        Roles::Miner => Some(Box::new(MinerBehavior::get(creep))),
    }
}
