use kondi::{textures::Textures, Context, GgezResult, util::{Point2, Vector2}};


#[derive(Debug, Clone)]
pub struct Rope {
    pub attach_point: Point2,
    // pub rot
    pub rope_force: f32
}

impl Rope {
    pub fn new(pos: Point2, force: f32) -> Self {
        Rope {
            attach_point: pos,
            rope_force: force,
        }
    }
    // pub fn update_force(pos: pos) {

    // }
}