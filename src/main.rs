use kondi::{ContextConfiguration, Context, Game, State, GameStateSetup, GgezResult, util::Point2, ggez::event::KeyCode};
use kondi::object::{
    tex_box::{TexBox, TexBoxData},
    ObjectId,
};
use ggez::{graphics::{Drawable, Color, Rect, MeshBuilder, Mesh, DrawMode, DrawParam, spritebatch::SpriteBatch}, GameResult};
use crate::{
    obj::player::{Player},
    utils::{
        animation::Animation,
        spritesheet::SpriteSheet,
        shape::{Shape, ShapeType},
    },
    game::{world2::{World, Level}},
};

pub mod utils;
pub mod obj;
pub mod game;

#[macro_use]
extern crate serde_derive;

pub mod util {
    use serde::{Deserializer, Deserialize};
}

fn main() {
    ContextConfiguration::new()
        .run::<EatTheStrawberries>()
        .unwrap()
}

struct EatTheStrawberries{
    player: ObjectId<Player>,
    world: World,
}

const UP: &'static str = "up";
const DOWN: &'static str = "down";
const LEFT: &'static str = "left";
const RIGHT: &'static str = "right";
const JUMP: &'static str = "jump";
const SLIDE: &'static str = "slide";

const SPEED: f32 = 100.;

impl Game for EatTheStrawberries {
    fn setup(ctx: &mut Context, s: &mut GameStateSetup<Self>) -> GgezResult<Self> {
        let (w, h) = s.dims();

        s.bind_keys(UP, vec![KeyCode::Up, KeyCode::W]);
        s.bind_keys(DOWN, vec![KeyCode::Down, KeyCode::S]);
        s.bind_keys(LEFT, vec![KeyCode::Left, KeyCode::A]);
        s.bind_keys(RIGHT, vec![KeyCode::Right, KeyCode::D]);
        s.bind_keys(JUMP, vec![KeyCode::Space]);
        s.bind_keys(SLIDE, vec![KeyCode::LShift]);

        let mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), 
            Rect{x:0.,y:0.,h: 35., w: 34.}, 
            Color{r: 0.5, g: 0.5, b: 0.5, a: 1.});
        // 32, 30
        // 70, 68
        let playersheet = SpriteSheet::new(
            "character/CharacterSpriteSheet", 
            s.state.textures.get_img(ctx,"character/CharacterSpriteSheet").dimensions(),
            vec!["running","idle"],
            vec![vec![
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
                    Rect::new(0., 35., 30., 34.),
                    Rect::new(30., 35., 30., 34.),
                    Rect::new(60., 35., 30., 34.)
                ]],
        );
        let mut level = Level::new_default();
        let world_img = s.state.textures.get_img(ctx, &level.palette.block_materials.spritesheet);
        // let worldbatch = SpriteBatch::new((*world_img).clone());
        
        let pos = Point2::new(w / 2.,h / 2.);
        // let floor_rect = Rect::new(
        //     pos.x,
        //     pos.y,
        //     64.,
        //     64.,
        // );
        let floor_rect2 = Rect::new(0., 0., 128., 64.,);
        let floor_mesh = Mesh::new_rectangle(ctx, DrawMode::fill(), floor_rect2,
            Color{r: 0.5, g: 0.5, b: 0.5, a: 1.});

        let floor_shape = Shape::new(
            pos, 0., floor_mesh?, ShapeType::Rectangle{rect: floor_rect2});
        
        level.add_static_shape(floor_shape,"white_brick");
        let player_rect = Rect::new(pos.x-17., pos.y-64., 35., 34.);

        let this_player = Player::new(
            ctx,
            player_rect,
            mesh?,
            playersheet
        );
        let player_id = s.object_set.add(this_player);
        // let _walking_box = s.object_set.add(TexBox::new(
            //     TexBoxData {
                //         texture: "box",
                //         pos: Point2::new(w / 2., h / 2.),
        //         rot: 0.,
        //     }, |data, ctx, state, delta| {
            //         if state.is_down(ctx, UP) {
                //             data.pos.y += SPEED * delta;
                //         }
                //         if state.is_down(ctx, DOWN) {
                    //             data.pos.y -= SPEED * delta;
                    //         }
                    //         if state.is_down(ctx, LEFT) {
        //             data.pos.x += SPEED * delta;
        //         }
        //         if state.is_down(ctx, RIGHT) {
            //             data.pos.x -= SPEED * delta;
            //         }
            //     }
            // ));
        
        Ok(EatTheStrawberries{
            player: player_id,
            world: {
                let mut world = World {
                    palette: level.palette,
                    // world_batch: worldbatch,
                    static_shapes: level.static_shapes,
                };
                // world.create_batch();
                world
            }
        })
    }
    fn logic(&mut self, _state: &mut State) -> GgezResult { 
        Ok(()) 
    }

    fn tick(&mut self, _state: &mut State) -> GgezResult { 
        Ok(()) 
    }

    fn draw(&self, ctx: &mut Context, _state: &State) -> GgezResult {
        let dp = DrawParam::default();
        self.world.draw_mat(ctx, dp)?;
        Ok(())
    }
    fn draw_hud(&self, _ctx: &mut Context, _state: &State) -> GgezResult { 
        Ok(()) 
    }
}
