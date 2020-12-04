use kondi::{textures::Textures, Context, State, GgezResult, util::Vector2, object::Object};

use ggez::graphics::Rect;
use crate::{
    utils::{util, util::Orientation, animation::AnimationContainer, spritesheet::SpriteSheet},
    obj::{shape::{Shape, ShapeType}, body::{Body, Material}},
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PlayerState {
    Idle,
    Running,
    Jumping,
    Falling,
    Crouched,
    Sliding,
    Crawling,
    OnWall,
}

impl PlayerState {
    pub fn get_from_str(state: &str) -> Option<PlayerState> {
        use self::PlayerState::*;
        let state = state.to_lowercase();
        match &*state {
            "idle" => Some(Idle),
            "running" => Some(Running),
            "jumping" => Some(Jumping),
            "falling" => Some(Falling),
            "crouched" => Some(Crouched),
            "crawling" => Some(Crawling),
            "sliding" => Some(Sliding),
            "on_wall" => Some(OnWall),
            _ => None,
        }
    }
    pub fn get_str(self) -> &'static str {
        use self::PlayerState::*;
        match self {
            Idle => "idle",
            Running => "running",
            Jumping => "jumping",
            Falling => "jumping",
            Crouched => "crouched",
            Crawling => "crouched",
            Sliding => "sliding",
            OnWall => "on_wall",
            // _ => "idle",
        }
    }
    pub fn is_crouched(self) -> bool {
        use self::PlayerState::*;
        match self {
            Crawling => true,
            Sliding => true,
            Crouched => true,
            _ => false,
        }        
    }

    pub fn can_crawl(self) -> bool {
        use self::PlayerState::*;
        match self {
            Crawling => true,
            Crouched => true,
            _ => false,
        }        
    }
    
    pub fn can_attach_wall(self) -> bool {
        use self::PlayerState::*;
        match self {
            Jumping => true,
            Falling => true,
            _ => false,
        }        
    }

    pub fn can_run(self) -> bool {
        use self::PlayerState::*;
        match self {
            Idle => true,
            Running => true,
            Falling => true,
            Jumping => true,
            _ => false,
        }
    }
    
    pub fn can_jump(self) -> bool {
        use self::PlayerState::*;
        match self {
            Jumping => false,
            Falling => false,
            _ => true,
        }
    }
    
    pub fn move_mult(self) -> f32 {
        use self::PlayerState::*;
        match self {
            Idle => 1.0,
            Running => 1.0,
            Jumping => 0.25,
            Falling => 0.25,
            Crouched => 0.65,
            Crawling => 0.65,
            Sliding => 0.,
            OnWall => 0.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Player {
    pub body: Body,
    pub hit_box: Vector2,
    pub crawl_hit_box: Vector2,
    pub state: PlayerState,
    pub spritesheet: SpriteSheet,
    pub anim_container: AnimationContainer,
    pub jump_vel: Vector2,
    pub movement_force: Vector2,
}

impl Player {
    pub fn new(_ctx: &mut Context, rect: Rect, s: SpriteSheet, a: AnimationContainer) -> Self {
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
                        static_friction: 0.8,
                        dynamic_friction: 0.65
                    },
                    5.0,
            ),
            spritesheet: s,
            hit_box: Vector2::new(rect.w, rect.h),
            crawl_hit_box: Vector2::new(rect.w, 20.),
            jump_vel: Vector2::new(0.,-224.),
            movement_force: Vector2::new(6.,0.),
            state: PlayerState::Idle,
            anim_container: a,
        }
    }
    pub fn jump(&mut self) {
        if self.body.velocity.y.abs() < 5.5 && self.state.can_jump() {
            let mut jump_vel = self.jump_vel;
            if self.state == PlayerState::Crouched {
                jump_vel *= 1.18;
            } else if self.state == PlayerState::Sliding {
                jump_vel *= 1.35;
                self.body.add_vel(-Vector2::new(self.body.velocity.x, 0.));
            } else if self.state == PlayerState::OnWall {
                self.switch_state(PlayerState::Jumping);
                if self.spritesheet.orientation == Orientation::Left {
                    jump_vel += 48.*self.movement_force+(-0.25*jump_vel);
                } else {
                    jump_vel += -48.*self.movement_force+(0.25*jump_vel);
                }
                self.spritesheet.switch_orientation();
            }
            self.body.add_vel(jump_vel);
            self.switch_state(PlayerState::Jumping);
        }
    }
    
    pub fn to_wall(&mut self, _ctx: &mut Context, _state: &mut State) {
        if (self.state.can_attach_wall()) || (self.state == PlayerState::OnWall && self.body.velocity.y.abs() < 5.5) {
            self.switch_state(PlayerState::OnWall);
            self.body.add_vel(Vector2::new(0.,-0.9*self.body.velocity.y));
            if self.spritesheet.orientation == Orientation::Left {
                self.body.add_vel(-self.movement_force);
            } else {
                self.body.add_vel(self.movement_force);
            }
        } else if self.state == PlayerState::OnWall {
            self.switch_state(PlayerState::Falling);
        }
    }

    pub fn state_control(&mut self, ctx: &mut Context, state: &mut State, col: f32) {
        if col.abs() == 1. {
            self.to_wall(ctx, state);
            return;
        }
        if col.abs() != 2. {
            return;
        }
        if self.body.velocity.y.abs() < 5.5 {
            if state.is_down(ctx, util::SHIFT) && self.body.velocity.x.abs() < 50.{
                self.switch_state(PlayerState::Crouched);
            } else if self.body.velocity.x.abs() < 50. {
                self.switch_state(PlayerState::Idle);
            }
        }
        if self.body.velocity.x.abs() > self.body.velocity.y.abs() {
            if self.state.can_run() {
                self.switch_state(PlayerState::Running);
            }
            if state.is_down(ctx, util::SHIFT) {
                if self.body.velocity.x.abs() > 50. && !self.state.can_crawl() {
                    self.switch_state(PlayerState::Sliding);
                } else {
                    self.switch_state(PlayerState::Crouched);
                }
            }
        }
    }
    
    pub fn switch_state(&mut self, to_state: PlayerState) {
        if !(self.state == to_state) {
            self.anim_container.change_image(to_state.get_str());
            self.state = to_state;
        }
    }

    // pub fn resolve_collision(&mut self, data: &CollisionData, ctx: &mut Context, state: &mut State){
    //     if data.normal.y == 1. && self.body.velocity.y.abs() < 5.5 && self.state == PlayerState::Jumping {
    //         // self.state_control(ctx, state);
    //     // } else if data.normal.x.abs() == 1. {
    //     //     self.to_wall(ctx, state);
    //     }
    // }
}

impl Object for Player {
    fn draw(&self, ctx: &mut Context, texes: &Textures) -> GgezResult<()> {
        // self.spritesheet.draw(ctx, texes, "idle", 0, self.body.shape.get_pos())
        let img = self.state.get_str();
        self.spritesheet.draw_with_offset(ctx, texes, img, self.anim_container.cur_step, self.body.shape.get_pos(), self.body.shape.dimensions())
    }

    fn update(&mut self, ctx: &mut Context, state: &mut State, delta: f32) {
        self.anim_container.update_timer();
        if state.is_down(ctx, util::SPACE) {
            self.jump();
        }
        if state.is_down(ctx, util::RIGHT) {
            self.body.add_vel(self.movement_force * self.state.move_mult());
            if self.state == PlayerState::OnWall {self.switch_state(PlayerState::Jumping);}
            self.spritesheet.change_orientation(Orientation::Right);
        }
        if state.is_down(ctx, util::LEFT) {
            self.body.add_vel(-self.movement_force * self.state.move_mult());
            if self.state == PlayerState::OnWall {self.switch_state(PlayerState::Jumping);}
            self.spritesheet.change_orientation(Orientation::Left);
        }
        if self.state != PlayerState::Jumping && self.state != PlayerState::OnWall {
            self.state_control(ctx, state, 0.);
        }

        if self.state.is_crouched() {
            self.body.shape.scale(self.crawl_hit_box);
            // self.body.shape.move_by(self.crawl_hit_box-self.hit_box);
        } else {
            self.body.shape.scale(self.hit_box);
        }
        self.body.update(delta);
    }
}