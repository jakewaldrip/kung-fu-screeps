use std::{collections::HashSet, str::FromStr};

use js_sys::{JsString, Object, Reflect};
use log::*;
use screeps::{game::{self}, RoomName};
use wasm_bindgen::prelude::*;

pub fn run_memory_manager() {
  info!("Running memory manager");

  if game::time() % 1000 == 0 {
    clean_memory();
  }
}

fn clean_memory() {
  info!("Running memory cleanup");

  // Remove all non-living creeps from memory
  let living_creeps: HashSet<String> = game::creeps().keys().into_iter().collect();
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


  // Remove dead rooms from memory
  let active_rooms: HashSet<RoomName> = game::rooms().keys().into_iter().collect(); 
  if let Ok(memory_rooms) = Reflect::get(&screeps::memory::ROOT, &JsString::from("creeps")) {
      let memory_rooms: Object = memory_rooms.unchecked_into();
      for room_name_js in Object::keys(&memory_rooms).iter() {
          let room_name = String::from(room_name_js.dyn_ref::<JsString>().unwrap());
          let room_name_struct = RoomName::from_str(&room_name).unwrap();

          if !active_rooms.contains(&room_name_struct) {
              info!("Deleting memory for inactive room {}", room_name);
              let _ = Reflect::delete_property(&memory_rooms, &room_name_js);
          }
      }
  }
}