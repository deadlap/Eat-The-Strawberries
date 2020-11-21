use kondi::{ContextConfiguration, Context, Game, 
    State, GameStateSetup, GgezResult, 
    util::Point2,
    ggez::event::KeyCode,
    object::ObjectSet,
};
use ggez::graphics::Rect;
use crate::{
    obj::{player::{Player, PlayerState}, shape::{Shape, ShapeType},
        body::{Body, Material}},
    utils::{spritesheet::SpriteSheet, collision::{CollisionData}, util},
    game::{world::{World, Level}},
};

pub mod utils;
pub mod obj;
pub mod game;

#[macro_use]
extern crate serde_derive;

fn main() {
    ContextConfiguration::new()
        .run::<EatTheStrawberries>()
        .unwrap()
}

struct EatTheStrawberries {
    player: Player,
    world: World,
}
const DESIRED_FPS: u32 = 60;

pub(crate) const DELTA: f32 = 1. / DESIRED_FPS as f32;

impl Game for EatTheStrawberries {
    fn setup(ctx: &mut Context, s: &mut GameStateSetup<Self>) -> GgezResult<Self> {
        let (w, h) = s.dims();
        let pos = Point2::new(w / 2.,h / 2.);
        
        s.bind_keys(util::UP, vec![KeyCode::Up, KeyCode::W]);
        s.bind_keys(util::JUMP, vec![KeyCode::Space]);
        s.bind_keys(util::DOWN, vec![KeyCode::Down, KeyCode::S]);
        s.bind_keys(util::LEFT, vec![KeyCode::Left, KeyCode::A]);
        s.bind_keys(util::RIGHT, vec![KeyCode::Right, KeyCode::D]);
        s.bind_keys(util::SLIDE, vec![KeyCode::LShift]);
        
        // s.add_key_press_handler(JUMP, Box::new(|_, game, _, _| {
        //     game.player.jump();
        //     Ok(())
        // }));

        // 32, 30
        // 70, 68
        let playersheet = SpriteSheet::new(
            "character/CharacterSpriteSheet", 
            s.state.textures.get_img(ctx,"character/CharacterSpriteSheet").dimensions(),
            vec!["idle","running", "jumping"],
            vec![
                vec![
                    Rect::new(0., 35., 30., 34.),
                    Rect::new(30., 35., 30., 34.),
                    Rect::new(60., 35., 30., 34.)
                ],
                vec![
                    Rect::new(1., 0., 20., 35.), 
                    Rect::new(22., 0., 23., 35.),
                    Rect::new(45., 0., 32., 35.),
                    Rect::new(77., 0., 34., 35.),
                    Rect::new(111., 0., 26., 35.),
                    Rect::new(140., 0., 22., 35.),
                    Rect::new(162., 0., 25., 35.),
                    Rect::new(190., 0., 30., 35.),
                    Rect::new(221., 0., 34., 35.),
                    Rect::new(255., 0., 29., 35.),
                ],
                vec![
                    Rect::new(90., 35., 24., 46.),
                    Rect::new(119., 35., 15., 46.),
                    Rect::new(140., 35., 19., 46.),
                    Rect::new(162., 35., 23., 46.),
                    Rect::new(187., 35., 27., 46.),
                    Rect::new(219., 35., 24., 46.),
                    Rect::new(244., 35., 30., 46.),
                ]
            ],
        );
        let player_rect = Rect::new(pos.x, pos.y-64., 30., 34.);
        let this_player = Player::new(
            ctx,
            player_rect,
            playersheet
        );

        let mut level = Level::new_default();

        let floor_rect = Rect::new(0., pos.y, w, 64.);
        let floor_shape = Shape::new(0., ShapeType::Rectangle{rect: floor_rect});
        let floor_body = Body::with_mass(floor_shape, Material {density: 2., restitution: 0.1}, 0., 0., 0.);

        let roof_rect = Rect::new(0., 0., w, 64.);
        let roof_shape = Shape::new(0., ShapeType::Rectangle{rect: roof_rect});
        let roof_body = Body::with_mass(roof_shape, Material {density: 2., restitution: 0.1}, 0., 0., 0.);

        let left_rect = Rect::new(0., 64., 32., pos.y-64.);
        let left_shape = Shape::new(0., ShapeType::Rectangle{rect: left_rect});
        let left_wall = Body::with_mass(left_shape, Material {density: 2., restitution: 0.1}, 0., 0., 0.);
        
        let right_rect = Rect::new(w-32., 64., 32., pos.y-64.);
        let right_shape = Shape::new(0., ShapeType::Rectangle{rect: right_rect});
        let right_wall = Body::with_mass(right_shape, Material {density: 2., restitution: 0.1}, 0., 0., 0.);
        
        level.add_static_shape(roof_body, "materials/brick_wall".to_string());
        level.add_static_shape(floor_body, "materials/concrete_wall".to_string());
        level.add_static_shape(left_wall, "materials/concrete_wall".to_string());
        level.add_static_shape(right_wall, "materials/concrete_wall".to_string());

        Ok(EatTheStrawberries {
            player: this_player,
            world: World {
                    palette: level.palette,
                    static_shapes: level.static_shapes,
                    dynamic_shapes: level.dynamic_shapes,
                },
        })
    }
    fn logic(&mut self, _ctx: &mut Context, state: &mut State) -> GgezResult {
        state.focus_on(self.player.body.shape.get_pos());
        Ok(())
    }

    fn tick(&mut self, ctx: &mut Context, state: &mut State) -> GgezResult {
        self.player.update(ctx, state, DELTA);
        for (mut body,..) in &self.world.static_shapes {
            if let Some(mut data) = CollisionData::check_collision(&mut self.player.body, &mut body) {
                data.positional_correction();
                data.resolve_collision();
                if data.normal.y == 1. && self.player.body.velocity.y.abs() > 0.1 {
                    self.player.switch_state(PlayerState::Idle);
                }
            }
        }
        Ok(())
    }

    fn draw(&self, ctx: &mut Context, state: &State, _: &ObjectSet) -> GgezResult {
        self.world.draw_mat(ctx, &state.textures)?;
        self.player.draw(ctx, &state.textures)?;
        Ok(())
    }
    fn draw_hud(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult { 
        Ok(()) 
    }
}
