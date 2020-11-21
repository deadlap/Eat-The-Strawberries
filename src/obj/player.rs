use kondi::{textures::Textures, Context, State, GgezResult, util::Vector2};

use ggez::graphics::Rect;
use crate::{
    utils::{util, spritesheet::SpriteSheet},
    obj::{shape::{Shape, ShapeType}, body::{Body, Material}},
    DELTA,
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
    pub body: Body,
    pub state: PlayerState,
    pub spritesheet: SpriteSheet,
    pub jump_vel: Vector2,
    pub movement_force: Vector2,
}

impl Player {
    pub fn new(_ctx: &mut Context, rect: Rect, s: SpriteSheet) -> Self {
        Self {
            body: Body::new(
                    Shape::new(
                        0.,
                        ShapeType::Rectangle {
                            rect: rect,
                        },
                    ),
                    Material {
                        density: 1.,
                        restitution: 0.6,
                    },
                    5.0,
                ),
            spritesheet: s,
            jump_vel: Vector2::new(0.,-50.),
            movement_force: Vector2::new(2.,0.),
            state: PlayerState::Idle,
        }
    }
    pub fn draw(&self, ctx: &mut Context, texes: &Textures) -> GgezResult<()> {
        let img: &str;
        match self.state {
            PlayerState::Idle => {
                img = "idle";
            },
            PlayerState::Jumping => {
                img = "jumping";
            },
            PlayerState::Running => {
                img = "running";
            },
            _ => {
                img = "idle";
            }
        }        
        self.spritesheet.draw_with_offset(ctx, texes, img, 0, self.body.shape.get_pos(), self.body.shape.dimensions())
    }
    
    pub fn update(&mut self, ctx: &mut Context, state: &mut State, delta: f32) {
        if state.is_down(ctx, util::UP) {
            self.jump();
        }
        if self.state != PlayerState::Jumping {
            if state.is_down(ctx, util::RIGHT) {
                self.body.add_vel(self.movement_force);
                // self.body.shape.move_by(self.movement_force/DELTA);
            }
            if state.is_down(ctx, util::LEFT) {
                self.body.add_vel(-self.movement_force);
                // self.body.shape.move_by(-self.movement_force/DELTA);
            }
        }
        self.body.update(delta);
        // if self.body.velocity.norm() < delta {
        //     self.switch_state(PlayerState::Idle);
        // }
    }
    
    pub fn jump(&mut self) {
        if self.body.velocity.y.abs() < 0.8 && self.state != PlayerState::Jumping {
            self.switch_state(PlayerState::Jumping);
            self.body.add_vel(self.jump_vel);
        }
    }
    
    pub fn switch_state(&mut self, to_state: PlayerState) {
        self.state = to_state;
    }
    // fn change_frc(&mut self, change: Vector2) {
    //     self.body.add_force(change);
    // }
    // fn change_acc(&mut self, change: Vector2) {
    //     let cur_x = self.body.force.x;
    // }
}

// impl Object for Player {
//     fn draw(&self, ctx: &mut Context, texes: &Textures) -> GgezResult<()> {
//         // self.spritesheet.draw(ctx, texes, "idle", 0, self.body.shape.get_pos())
//         Ok(())
//     }

//     fn update(&mut self, ctx: &mut Context, state: &mut State, delta: f32) {
//         // self.body.update(ctx, state, delta)
//     }
// }