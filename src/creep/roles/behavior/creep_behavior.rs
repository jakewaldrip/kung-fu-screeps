use std::{cell::RefCell, collections::HashMap};

use screeps::Room;

use crate::job::job::Job;

thread_local! {
    pub static CREEP_JOB: RefCell<HashMap<String, Job>> = RefCell::new(HashMap::new());
}

pub trait CreepBehavior {
    fn get_job(&self, room: &Room) -> ();
    fn do_job(&self, room: &Room) -> ();
}
