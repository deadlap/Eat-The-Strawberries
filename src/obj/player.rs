use kondi::{textures::Textures, Context, State, GgezResult, util::{Point2, Vector2}, ggez::event::KeyCode};
use kondi::object::Object;

use ggez::{graphics::{Rect, Mesh}};
use crate::{
    utils::{animation::Animation, spritesheet::SpriteSheet, shape::{Shape, ShapeType}},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
    Falling,
    Sliding,
    // StartJump,
    // Walking,
    // EndJump,
    // Stopping,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub shape: Shape,
    // pub sprites: SpriteBatch,
    pub state: PlayerState,
    pub vel: Vector2,
    // pub applied_f: Vector2;
    pub spritesheet: SpriteSheet,
}

impl Player {
    #[inline]
    pub fn new(ctx: &mut Context, rect: Rect, mesh: Mesh, s: SpriteSheet) -> Self {
        Self {
            shape: Shape::new(
                Point2::new(rect.x, rect.y),
                0., 
                mesh,
                ShapeType::Rectangle {
                    rect: rect,
                },
            ),
            spritesheet: s,
            state: PlayerState::Idle,
            vel: Vector2::new(0.,0.),
        }
    }
    fn switch_state(&mut self, to_state: PlayerState) {
        self.state = to_state;
    }
}

impl Object for Player {
    fn draw(&self, ctx: &mut Context, texes: &Textures) -> GgezResult<()> {
        self.spritesheet.draw(ctx, texes, "idle", 0, self.shape.pos)
    }

    fn update(&mut self, ctx: &mut Context, state: &mut State, delta: f32) {
        
    }
}