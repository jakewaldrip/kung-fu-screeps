use screeps::{game, OwnedStructureProperties, Room};

pub fn get_owned_rooms() -> Vec<Room> {
    game::rooms()
        .values()
        .filter(|room| {
            let controller = room.controller();
            match controller {
                Some(controller) => controller.my(),
                None => false,
            }
        })
        .collect()
}
