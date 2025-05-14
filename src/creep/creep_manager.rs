use log::info;
use screeps::{game, SharedCreepProperties};

use crate::creep::roles::roles_api::{get_creep_behavior_impl, get_creep_data_impl, Roles};

pub fn run_creep_manager() {
    for creep in game::creeps().values() {
        info!("Running creep {}", creep.name());

        let role: Roles = Roles::Miner;
        let creep_behavior = get_creep_behavior_impl(&role, creep).unwrap();
        let creep_data = get_creep_data_impl(&role).unwrap();

        info!("Creeps name is {}", creep_data.get_name());
        creep_behavior.say();
    }
}
