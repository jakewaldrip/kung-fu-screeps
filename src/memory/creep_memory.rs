use screeps::Creep;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

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

    pub fn to_js(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap()
    }
}
