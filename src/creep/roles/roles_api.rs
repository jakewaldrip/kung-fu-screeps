use std::fmt::Display;

use screeps::Creep;
use serde::{Deserialize, Serialize};

use crate::creep::roles::{behavior::miner::MinerBehavior, data::miner::MinerData};

use super::{behavior::creep_behavior::CreepBehavior, data::creep_data::CreepData};

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Clone)]
pub enum Roles {
    Miner,
}

impl Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Roles::Miner => write!(f, "Miner"),
        }
    }
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
