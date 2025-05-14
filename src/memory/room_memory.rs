use screeps::Room;
use serde::{Deserialize, Serialize};

const ROOM_MEMORY_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomMemory {
    version: u32,
    room_state: RoomState,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RoomState {
    BOOTSTRAP,
    BASIC,
}

impl RoomMemory {
    pub fn init(room: Room) {
        let room_memory = Self {
            version: ROOM_MEMORY_VERSION,
            room_state: RoomState::BOOTSTRAP,
        };

        let current_room_memory_result: Result<RoomMemory, _> =
            serde_wasm_bindgen::from_value(room.memory());

        match current_room_memory_result {
            Ok(current_room_memory) => {
                if current_room_memory.version != ROOM_MEMORY_VERSION {
                    room.set_memory(&serde_wasm_bindgen::to_value(&room_memory).unwrap());
                }
            }
            Err(_) => {
                room.set_memory(&serde_wasm_bindgen::to_value(&room_memory).unwrap());
            }
        }
    }

    pub fn set_room_state(room: Room, room_state: RoomState) {
        let mut current_room_memory: RoomMemory =
            serde_wasm_bindgen::from_value(room.memory()).unwrap();
        current_room_memory.room_state = room_state;
        room.set_memory(&serde_wasm_bindgen::to_value(&current_room_memory).unwrap());
    }
}
