use kondi::{Context, Game, 
    State, GameStateSetup, GgezResult, 
    util::Point2,
    ggez::event::KeyCode,
    object::{ObjectSet, ObjectId},
};
use ggez::graphics::Rect;
use crate::{
    obj::{player::Player, shape::{Shape, ShapeType},
        body::{Body, Material}},
    utils::{spritesheet::SpriteSheet, animation::AnimationContainer, collision::{CollisionData}, util},
    game::{world::{World, Level}},
};

pub struct PlayState {
    player_id: ObjectId<Player>,
    world_id: ObjectId<World>,
}

impl Game for PlayState {
    fn setup(ctx: &mut Context, s: &mut GameStateSetup<Self>) -> GgezResult<Self> {
        let (w, h) = s.dims();
        let pos = Point2::new(w / 2.,h / 2.);
        
        s.bind_keys(util::UP, vec![KeyCode::Up, KeyCode::W]);
        s.bind_keys(util::SPACE, vec![KeyCode::Space]);
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

        let player = s.object_set.add(this_player);
        // level.save("levels/level.lvl").unwrap();
        let level = Level::load("levels/level.lvl").unwrap();
        let world = s.object_set.add(
            World {
                palette: level.palette,
                static_shapes: level.static_shapes,
                dynamic_shapes: level.dynamic_shapes,
            });
        
        Ok(PlayState {
            player_id: player,
            world_id: world,
        })
    }
    fn logic(&mut self, _ctx: &mut Context, _state: &mut State, _: &mut ObjectSet) -> GgezResult {
        Ok(())
    }

    fn tick(&mut self, ctx: &mut Context, state: &mut State, object_set: &mut ObjectSet, _delta: f32) -> GgezResult {
        state.focus_on(object_set.get(self.player_id).unwrap().body.shape.get_pos());
        for (mut body,..) in object_set.get_mut(self.world_id).unwrap().static_shapes.clone() {
            if let Some(mut data) = CollisionData::check_collision(&mut object_set.get_mut(self.player_id).unwrap().body, &mut body) {
                data.positional_correction();
                data.resolve_collision();
                let bob = if data.normal.y == 1. {2.} else {data.normal.x};
                object_set.get_mut(self.player_id).unwrap().state_control(ctx, state, bob);
            }
        }
        Ok(())
    }

    fn draw(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult {
        Ok(())
    }
    fn draw_hud(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult { 
        Ok(()) 
    }
}