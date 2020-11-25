pub mod collision;
pub mod spritesheet;
pub mod animation;

pub mod util {
    pub const GRAVITY: f32 = 64.;
    pub const UP: &'static str = "up";
    pub const DOWN: &'static str = "down";
    pub const LEFT: &'static str = "left";
    pub const RIGHT: &'static str = "right";
    pub const JUMP: &'static str = "jump";
    pub const SHIFT: &'static str = "slide";
    #[derive(Debug, Clone, PartialEq)]
    pub enum Orientation {
        Left,
        Right,
    }
}