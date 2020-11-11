use kondi::{
    util::{Point2, Vector2},
    textures::Textures,
    GgezResult,
};

use ggez::{
    Context, GameResult,
    error::GameError,
    graphics::{self, spritebatch::{SpriteBatch, SpriteIdx}, DrawParam},
};

use crate::{
    util,
    obj::{player::Player},
    utils::{spritesheet::SpriteSheet, shape::Shape}
};

use std::path::Path;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

use bincode;

mod material2;
pub use material2::*;

#[derive(Debug, Clone)]
pub struct World {
    // pub player: Player,
    pub palette: Palette,
    // pub grid: Grid,
    // pub world_batch: SpriteBatch,
    pub static_shapes: Vec<(Shape,String)>,
    // pub dynamic_shapes: Vec<Shape>,
    // pub exit: Option<Point2>,
    // pub exit: Option<Point2>,
    // pub intels: Vec<Point2>,
}

impl World {
    // pub fn create_batch(&mut self) {
    //     for (shape, sprite) in &self.static_shapes {
    //         self.world_batch.add(
    //             (self.palette.block_materials.spriteparts.get(sprite).unwrap())[0].dest(shape.pos)
    //         );
    //     }
    // }
    pub fn draw_mat(&self, ctx: &mut Context, dp: DrawParam) -> GgezResult<()> {
        // graphics::draw(ctx, &self.world_batch, dp)
        for (shape, name) in &self.static_shapes {
            // graphics::draw(ctx, dp)
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub palette: Palette,
    // pub grid: Grid,
    pub static_shapes: Vec<(Shape,String)>,
    // pub dynamic_shapes: Vec<Shape>,
    // pub start_point: Option<Point2>,
    // pub exit: Option<Point2>,
    // pub intels: Vec<Point2>,
}

impl Level {
    pub fn new(palette: Palette) -> Self {//, width: u16, height: u16) -> Self {
        Self {
            palette,
            // grid: Grid::new(width, height),
            static_shapes: Vec::new(),
            // dynamic_shapes: Vec::new(),
            // start_point: None,
            // exit: None,
            // intels: Vec::new(),
        }
    }
    pub fn new_default() -> Self {//, width: u16, height: u16) -> Self {
        Self {
            palette: Palette::default(),
            // grid: Grid::new(width, height),
            static_shapes: Vec::new(),
            // dynamic_shapes: Vec::new(),
            // start_point: None,
            // exit: None,
            // intels: Vec::new(),
        }
    }
    pub fn add_static_shape(&mut self, sh: Shape, name: &str) {
        self.static_shapes.push((sh, name.to_string()))
    }
    // pub fn add_static_shape_vec(&self, ) {

    // }
    // pub fn add_static_shape(&self, ) {

    // }
    // pub fn add_static_shape_vec(&self, ) {

    // }
}




// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Grid{
//     width: u16,
//     mats: Vec<u8>,
// }

// impl Grid {
//     pub fn new(width: u16, height: u16) -> Self {
//         Grid {
//             width,
//             mats: vec![0; (width*height) as usize],
//         }
//     }
//     #[inline]
//     pub fn width(&self) -> u16 {
//         self.width
//     }
//     pub fn height(&self) -> u16 {
//         self.mats.len() as u16 / self.width
//     }
//     pub fn widen(&mut self) {
//         let width = self.width as usize;
//         let height = self.height() as usize;
//         self.mats.reserve_exact(height);
//         for i in (1..=height).rev().map(|i| i * width) {
//             self.mats.insert(i, 0);
//         }
//         self.width += 1;
//     }
//     pub fn thin(&mut self) {
//         if self.width <= 1 {
//             return
//         }
//         let width = self.width;
//         for i in (1..=self.height()).rev().map(|i| i * width - 1) {
//             self.mats.remove(i as usize);
//         }
//         self.width -= 1;
//     }
//     pub fn heighten(&mut self) {
//         let new_len = self.mats.len() + self.width as usize;
//         self.mats.reserve_exact(self.width as usize);
//         self.mats.resize(new_len, 0);
//     }
//     pub fn shorten(&mut self) {
//         let new_len = self.mats.len() - self.width as usize;
//         if new_len == 0 {
//             return
//         }
//         self.mats.truncate(new_len);
//     }
//     #[inline]
//     pub fn snap(c: Point2) -> (u16, u16) {
//         Self::snap_coords(c.x, c.y)
//     }
//     #[inline]
//     fn idx(&self, x: u16, y: u16) -> usize {
//         x.saturating_add(y.saturating_mul(self.width)) as usize
//     }
//     pub fn snap_coords(x: f32, y: f32) -> (u16, u16) {
//         fn db32omin(n: f32) -> u16 {
//             if n < 0. {
//                 std::u16::MAX
//             } else {
//                 (n / 32.) as u16
//             }
//         }

//         (db32omin(x), db32omin(y))
//     }
//     pub fn get(&self, x: u16, y: u16) -> Option<u8> {
//         if x < self.width {
//             self.mats.get(self.idx(x, y)).cloned()
//         } else {
//             None
//         }
//     }
//     #[inline(always)]
//     pub fn is_solid_tuple(&self, pal: &Palette, (x, y): (u16, u16)) -> bool {
//         self.is_solid(pal, x, y)
//     }
//     pub fn is_solid(&self, pal: &Palette, x: u16, y: u16) -> bool {
//         self.get(x, y).map(|m| pal.is_solid(m)).unwrap_or(true)
//     }
//     pub fn insert(&mut self, x: u16, y: u16, mat: u8) {
//         if x < self.width {
//             let i = self.idx(x, y);
//             if let Some(m) = self.mats.get_mut(i) {
//                 *m = mat;
//             }
//         }
//     }
// }