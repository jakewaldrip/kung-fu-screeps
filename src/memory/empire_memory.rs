use js_sys::{JsString, Reflect};
use serde::{Deserialize, Serialize};

const EMPIRE_MEMORY_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct EmpireMemory {
    version: u32,
}

impl EmpireMemory {
    pub fn init() {
        let empire_memory = Self {
            version: EMPIRE_MEMORY_VERSION,
        };

        let empire_js_result = Reflect::get(&screeps::memory::ROOT, &JsString::from("empire"));
        let empire_js = match empire_js_result {
            Ok(empire_js) => empire_js,
            Err(_) => {
                Reflect::set(
                    &screeps::memory::ROOT,
                    &JsString::from("empire"),
                    &serde_wasm_bindgen::to_value(&empire_memory).unwrap(),
                )
                .unwrap();
                Reflect::get(&screeps::memory::ROOT, &JsString::from("empire")).unwrap()
            }
        };

        let current_empire_memory_result: Result<EmpireMemory, _> =
            serde_wasm_bindgen::from_value(empire_js);

        match current_empire_memory_result {
            Ok(current_empire_memory) => {
                if current_empire_memory.version != EMPIRE_MEMORY_VERSION {
                    Reflect::set(
                        &screeps::memory::ROOT,
                        &JsString::from("empire"),
                        &serde_wasm_bindgen::to_value(&empire_memory).unwrap(),
                    )
                    .unwrap();
                }
            }
            Err(_) => {
                Reflect::set(
                    &screeps::memory::ROOT,
                    &JsString::from("empire"),
                    &serde_wasm_bindgen::to_value(&empire_memory).unwrap(),
                )
                .unwrap();
            }
        }
    }
}
