#![allow(deprecated)]

use log::*;
use memory::memory_manager::run_memory_manager;
use screeps::{game::{self}};
use wasm_bindgen::prelude::*;

mod logging;
mod memory;

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

    run_memory_manager();

    debug!("Loop running! CPU: {}", game::cpu::get_used());
    info!("Done! cpu: {}", game::cpu::get_used())
}
