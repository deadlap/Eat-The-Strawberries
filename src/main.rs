use kondi::{ContextConfiguration};

pub mod utils;
pub mod obj;
pub mod game;

pub use game::states::*;

#[macro_use]
extern crate serde_derive;
fn main() {
    ContextConfiguration::new()
    .run::<play::PlayState>()
    .unwrap()
}

const DESIRED_FPS: u32 = 60;

pub(crate) const DELTA: f32 = 1. / DESIRED_FPS as f32;