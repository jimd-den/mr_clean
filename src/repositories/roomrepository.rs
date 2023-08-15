use crate::entities::room::RoomEntity;

pub trait RoomRepository {
    fn add_room(&mut self, room: &RoomEntity) -> Result<(), String>;
    fn get_room(&mut self, id: &u32) -> Option<RoomEntity>;
}
