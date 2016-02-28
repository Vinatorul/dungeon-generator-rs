/// Type of the room
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomType {
    /// Basic room is the room where you'd better to plase enemies, treasures, bosses, etc
    BasicRoom,
    /// Coridor is a connecter Cave-BasicRoom or BasicRoom-BasicRoom
    Coridor,
    /// Cave is "main hall" of a dungeon, may not present in some generator type such as BSP
    Cave,
}

/// Room description struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Room {
    /// The type of the room
    pub room_type: RoomType,
    /// x absolute coordinate
    pub x: u32,
    /// y absolute coordinate
    pub y: u32,
    /// width of the room in units
    pub width: u32,
    /// height of the room in units
    pub height: u32,
}

pub type Rectangle = [u32; 4];

pub trait DungeonGenerator {
    /// width and height should be passed in units (for example tile size), not in pixels
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
