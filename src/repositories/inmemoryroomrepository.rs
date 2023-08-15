use std::collections::HashMap;
use crate::entities::room::RoomEntity;
use crate::repositories::roomrepository::RoomRepository;

pub struct InMemoryRoomRepository {
    rooms: HashMap<u32, RoomEntity>
}

impl InMemoryRoomRepository {
    pub fn new() -> Self {
        let rooms = HashMap::new();
        InMemoryRoomRepository { rooms }
    }
}

impl RoomRepository for InMemoryRoomRepository {
    pub fn add_room(&mut self, room: &RoomEntity) -> Result<(), String> {
        self.rooms.insert(room.id, room.clone());
        Ok(())
    }

    pub fn get_room(&mut self, id: &u32) -> Option<RoomEntity> {
        self.rooms.get(id).cloned()
    }
} 
