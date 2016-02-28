extern crate rand;

mod core;
mod bsp_generator;

pub use core::{DungeonGenerator, RoomType, Room};
pub use bsp_generator::BSPGenerator;

#[test]
fn it_works() {
}
