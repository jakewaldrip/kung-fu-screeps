use std::{cell::RefCell, collections::HashMap};

use screeps::{find, game, HasId, ObjectId, Room, RoomName, Source, StructureSpawn};

thread_local! {
    pub static ROOM_CACHE: RefCell<HashMap<RoomName, RoomCache>> = RefCell::new(HashMap::new());
}

// TODO: Consider adding creeps to the cache
pub struct RoomCache {
    pub room_name: RoomName,

    pub sources: Vec<ObjectId<Source>>,
    pub spawns: Vec<ObjectId<StructureSpawn>>,
}

impl RoomCache {
    /// Creates a RoomCache entry in ROOM_CACHE for the room, overwriting previous value
    pub fn update_for_room(room: &Room) {
        ROOM_CACHE.with(|room_cache_ref| {
            let mut room_cache_map = room_cache_ref.borrow_mut();
            let room_name = room.name();

            // TODO: Change update_cache to accept config for individual object update times
            if let Some(room_cache) = room_cache_map.get_mut(&room_name) {
                if game::time() % 10 == 0 {
                    room_cache.update_cache(room);
                } else {
                    room_cache.validate_cache();
                }
            } else {
                let mut room_cache = RoomCache::new(room);
                room_cache.update_cache(room);
                room_cache_map.insert(room_name, room_cache);
            }
        });
    }

    /// Creates object with empty vecs
    /// room_name is set correctly
    pub fn new(room: &Room) -> Self {
        RoomCache {
            room_name: room.name(),
            sources: Vec::new(),
            spawns: Vec::new(),
        }
    }

    /// Removes dead items from cache
    /// Dead is defined as not having a result from game::get_object_by_id
    pub fn validate_cache(&mut self) {
        self.spawns.retain(|spawn_id| {
            let spawn = game::get_object_by_id_typed(spawn_id);
            spawn.is_some()
        });
    }

    /// Populate the cache with updated values from the room
    /// Uses room.find to locate structures
    pub fn update_cache(&mut self, room: &Room) {
        self.sources = room
            .find(find::SOURCES, None)
            .iter()
            .map(|source| source.id())
            .collect();

        self.spawns = game::spawns()
            .values()
            .filter_map(|spawn| {
                if spawn.room().unwrap().name() == self.room_name {
                    Some(spawn.id())
                } else {
                    None
                }
            })
            .collect();
    }
}
