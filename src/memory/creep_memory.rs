use screeps::Creep;
use serde::{Deserialize, Serialize};

use crate::creep::roles::roles_api::Roles;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreepMemory {
    pub home_room: String,
    pub role: Roles,
}

impl CreepMemory {
    pub fn get(creep: &Creep) -> Self {
        serde_wasm_bindgen::from_value(creep.memory()).unwrap()
    }
}
