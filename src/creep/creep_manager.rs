use screeps::game;

use crate::creep::roles::roles_api::{get_creep_behavior_impl, Roles};

pub fn run_creep_manager() {
    for creep in game::creeps().values() {
        if creep.spawning() {
            return ();
        }

        let role: Roles = Roles::Miner;
        let creep_behavior = get_creep_behavior_impl(&role, creep).unwrap();
        creep_behavior.say();
    }
}
