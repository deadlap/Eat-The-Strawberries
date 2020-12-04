// use kondi::{Context, Game, 
//     State, GameStateSetup, GgezResult, 
//     util::Point2,
//     ggez::event::KeyCode,
//     object::{ObjectSet, ObjectId},
// };
// use ggez::graphics::Rect;
// use crate::{
//     obj::{player::Player, shape::{Shape, ShapeType},
//         body::{Body, Material}},
//     utils::{spritesheet::SpriteSheet, animation::AnimationContainer, collision::{CollisionData}, util},
//     game::{world::{World, Level}},
// };

// pub struct EditorState {
    
// }

// impl Game for EditorState {
//     fn setup(ctx: &mut Context, state: &mut GameStateSetup<Self>) -> GgezResult<Self> {

//     }
//     fn logic(&mut self, _: &mut Context, _: &mut State, _: &mut ObjectSet) -> GgezResult {
//         Ok(())
//     }
//     fn tick(&mut self, _: &mut Context, _: &mut State, _: &mut ObjectSet, _delta: f32) -> GgezResult {
//         Ok(())
//     }
//     fn draw(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult {
//         Ok(())
//     }
//     fn draw_hud(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult {
//         Ok(())
//     }
// }