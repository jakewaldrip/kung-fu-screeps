#![allow(deprecated)]

use creep::creep_manager::run_creep_manager;
use log::*;
use memory::memory_manager::run_memory_manager;
use room::room_manager::run_room_manager;
use screeps::game::{self};
use spawn::spawn_manager::run_spawn_manager;
use wasm_bindgen::prelude::*;

mod creep;
mod job;
mod logging;
mod memory;
mod room;
mod spawn;

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

    run_memory_manager();
    run_room_manager();
    run_spawn_manager();
    run_creep_manager();

    info!("Done! cpu: {}", game::cpu::get_used())
}
