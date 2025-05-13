pub struct CustomRoomMemory {
  room_state: RoomState,
}

pub enum RoomState {
  BOOTSTRAP,
  BASIC,
}