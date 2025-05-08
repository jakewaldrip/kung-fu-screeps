use std::collections::HashSet;

use js_sys::{JsString, Object, Reflect};
use log::*;
use screeps::game;
use wasm_bindgen::prelude::*;

mod logging;

static INIT_LOGGING: std::sync::Once = std::sync::Once::new();

// Using wasm heap example
// thread_local! {
//     static CREEP_TARGETS: RefCell<HashMap<String, CreepTarget>> = RefCell::new(HashMap::new());
// }

#[wasm_bindgen(js_name = loop)]
pub fn game_loop() {
    INIT_LOGGING.call_once(|| {
        logging::setup_logging(logging::Info);
    });

    debug!("Loop running! CPU: {}", game::cpu::get_used());

    if game::time() % 1000 == 0 {
        clean_memory();
    }

    info!("Done! cpu: {}", game::cpu::get_used())
}

fn clean_memory() {
    info!("Running memory cleanup");
    let mut living_creeps = HashSet::new();
    for creep_name in game::creeps().keys() {
        living_creeps.insert(creep_name);
    }

    // Remove all non-living creeps from memory
    if let Ok(memory_creeps) = Reflect::get(&screeps::memory::ROOT, &JsString::from("creeps")) {
        let memory_creeps: Object = memory_creeps.unchecked_into();
        for creep_name_js in Object::keys(&memory_creeps).iter() {
            let creep_name = String::from(creep_name_js.dyn_ref::<JsString>().unwrap());

            if !living_creeps.contains(&creep_name) {
                info!("deleting memory for dead creep {}", creep_name);
                let _ = Reflect::delete_property(&memory_creeps, &creep_name_js);
            }
        }
    }
}
