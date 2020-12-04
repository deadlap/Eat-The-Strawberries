// use kondi::{Game, Context, State, GameStateSetup, GgezResult,
//     util::Point2,
//     ggez::event::KeyCode,
//     object::{ObjectSet, ObjectId},
// };
// use std::mem;

pub mod world;
pub mod states;

pub use world::*;
pub use states::*;

pub enum StateSwitch {
    Menu,
    Play(Level),
    // Editor(Option<Level>),
    // PlayWith{
    //     lvl: Box<Level>,
    //     health: Health,
    // },
}

// pub enum GameState {
//     Play,
//     Menu,
//     // Editor,
// }

// pub struct StateMachine {
//     switch_state: Option<StateSwitch>,
//     gs: Box<dyn Game>,
//     // gs: GameState,
// }

// impl Default for StateMachine {
//     fn default() -> Self {
//         StateMachine {
//             switch_state: None,
//         }
//     }
// }

// impl Game for StateMachine {
//     fn setup(ctx: &mut Context, state: &mut GameStateSetup<Self>) -> GgezResult<Self> {
//         Ok(StateMachine::default())
//     }

//     fn logic(&mut self, _: &mut Context, _: &mut State, _: &mut ObjectSet) -> GgezResult { 
//         // match self.switch_state
//         if let Some(gsb) = mem::replace(&mut self.switch_state, None) {
//             self.gs = match self.gsb {
//                 GameState::Play(level) => states::play::PlayState,
//                 GameState::Menu => 
//             }
//         }
//         Ok(())
//     }
//     fn tick(&mut self, _: &mut Context, _: &mut State, _: &mut ObjectSet, _delta: f32) -> GgezResult { 
//         Ok(())
//     }
//     /// This function should draw other things on the screen
//     /// that follow the offset
//     fn draw(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult {
//         Ok(())
//     }
//     /// This should draw things on top of the what's drawn in `draw`
//     /// and that do not follow the offset
//     fn draw_hud(&self, _ctx: &mut Context, _state: &State, _: &ObjectSet) -> GgezResult {
//         Ok(())
//     }
// }