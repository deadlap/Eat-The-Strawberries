use kondi::{ContextConfiguration, Context, Game, 
    State, GameStateSetup, GgezResult, 
    util::Point2,
    ggez::event::KeyCode,
    object::ObjectSet,
};
use ggez::graphics::Rect;
use crate::{
    obj::{player::Player, shape::{Shape, ShapeType},
        body::{Body, Material}},
    utils::{spritesheet::SpriteSheet, animation::AnimationContainer, collision::{CollisionData}, util},
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
        s.bind_keys(util::SHIFT, vec![KeyCode::LShift]);
        
        let playersheet = SpriteSheet::new(
            "character/CharacterSpriteSheet", 
            s.state.textures.get_img(ctx,"character/CharacterSpriteSheet").dimensions(),
            vec!["idle", "running", "jumping", "crouched", "sliding", "on_wall"],
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
                ],
                vec![
                    Rect::new(284., 9., 25., 27.),
                ],
                vec![
                    Rect::new(274., 38., 42., 28.)
                ],
                vec![
                    Rect::new(312., 0., 28., 42.)
                ]
            ],
        );
        let anim_cont = AnimationContainer::new(
            vec!["idle", "running", "jumping", "crouched", "sliding", "on_wall"],
            vec![(0.45, 2), (0.3, 9), (0.3,6), (1.,1), (1.,1), (1.,1)]    
        );
        let player_rect = Rect::new(pos.x, pos.y-64., 30., 32.);
        let this_player = Player::new(
            ctx,
            player_rect,
            playersheet,
            anim_cont
        );

        let mut level = Level::new_default();

        let floor_rect = Rect::new(-w, pos.y, w*3., 32.);
        let floor_shape = Shape::new(0., ShapeType::Rectangle{rect: floor_rect});
        let floor_body = Body::with_mass(floor_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let roof_rect = Rect::new(0., 32., w, 32.);
        let roof_shape = Shape::new(0., ShapeType::Rectangle{rect: roof_rect});
        let roof_body = Body::with_mass(roof_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let left_rect = Rect::new(0., 64., 32., pos.y-128.);
        let left_shape = Shape::new(0., ShapeType::Rectangle{rect: left_rect});
        let left_wall = Body::with_mass(left_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);
        
        let right_rect = Rect::new(w-32., 64., 32., pos.y-88.);
        let right_shape = Shape::new(0., ShapeType::Rectangle{rect: right_rect});
        let right_wall = Body::with_mass(right_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let platform_rect = Rect::new(32., pos.y-96., 128., 32.);
            let platform_shape = Shape::new(0., ShapeType::Rectangle{rect: platform_rect});
            let platform_body = Body::with_mass(platform_shape, 
                Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let platform_rect2 = Rect::new(w-288., pos.y-64., 64., 64.);
        let platform_shape2 = Shape::new(0., ShapeType::Rectangle{rect: platform_rect2});
        let platform_body2 = Body::with_mass(platform_shape2, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);
        
        let platform_rect3 = Rect::new(224., pos.y-128., 32., 64.);
        let platform_shape3 = Shape::new(0., ShapeType::Rectangle{rect: platform_rect3});
        let platform_body3 = Body::with_mass(platform_shape3, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        level.add_static_shape(roof_body, "materials/brick_wall".to_string());
        level.add_static_shape(floor_body, "materials/concrete_wall".to_string());
        level.add_static_shape(left_wall, "materials/concrete_wall".to_string());
        level.add_static_shape(right_wall, "materials/concrete_wall".to_string());
        level.add_static_shape(platform_body, "materials/sidewalk".to_string());
        level.add_static_shape(platform_body2, "materials/brick_wall".to_string());
        level.add_static_shape(platform_body3, "materials/concrete_wall".to_string());


        Ok(EatTheStrawberries {
            player: this_player,
            world: World {
                    palette: level.palette,
                    static_shapes: level.static_shapes,
                    dynamic_shapes: level.dynamic_shapes,
                },
        })
    }
    fn logic(&mut self, _ctx: &mut Context, _state: &mut State) -> GgezResult {
        Ok(())
    }

    fn tick(&mut self, ctx: &mut Context, state: &mut State) -> GgezResult {
        state.focus_on(self.player.body.shape.get_pos());
        self.player.update(ctx, state, DELTA);
        for (mut body,..) in &self.world.static_shapes {
            if let Some(mut data) = CollisionData::check_collision(&mut self.player.body, &mut body) {
                data.positional_correction();
                data.resolve_collision();
                let bob = if data.normal.y == 1. {2.} else {data.normal.x};
                self.player.state_control(ctx, state, bob);
                // if data.normal.y == 1. && self.player.body.velocity.y.abs() < 5.5 && self.player.state == PlayerState::Jumping {
                // }
                // self.player.resolve_collision(&data, ctx, state);
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
