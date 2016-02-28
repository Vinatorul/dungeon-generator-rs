#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    BasicRoom,
    Coridor,
    Cave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    pub room_type: RoomType,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub type Rectangle = [u32; 4];

pub trait DungeonGenerator {
    fn generate(&mut self, seed: &[usize], width: u32, height: u32) -> Vec<Room>;
}

impl Room {
    pub fn new(room_type: RoomType, rect: Rectangle) -> Room {
        Room {
            room_type: room_type,
            x: rect[0],
            y: rect[1],
            width: rect[2],
            height: rect[3],
        }
    }
}
