use kondi::{util::Point2};

use ggez::graphics::{Mesh, Rect};

#[derive(Debug, Clone, Copy)]
// #[serde(rename_all = "lowercase")]
pub enum ShapeType{
    Rectangle {
        rect: Rect,
    },
    Circle {
        radius: f32,
    },
}

#[derive(Debug, Clone)]
pub struct Shape {
    pub pos: Point2,
    pub rot: f32,
    pub mesh: Mesh,
    pub shape: ShapeType,
}

impl Shape {
    pub fn new(pos: Point2, rot: f32, mesh: Mesh, shape: ShapeType) -> Self {
        Shape {
            pos,
            rot,
            mesh,
            shape,
        }
    }
    // fn draw(&self, ctx: &mut Context, texes: &Textures) -> GameResult<()> {
        
    // }
    // fn update(&mut self, ctx: &mut Context, state: &mut State, delta: f32) {

    // }
    pub fn move_pos(&mut self, change: Point2) {
        self.pos.x += change.x;
        self.pos.y += change.y;
    }
    pub fn set_pos(&mut self, change: Point2) {
        self.pos = change;
    }
    pub fn rotate(&mut self, change: f32) {
        self.rot += change;
    }
}