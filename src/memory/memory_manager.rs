use log::*;
use screeps::game::{self};

use crate::memory::memory_utils::clean_memory;

pub fn run_memory_manager() {
    info!("Running memory manager");

    if game::time() % 1000 == 0 {
        clean_memory();
    }
}
