use kondi::{
    GgezResult, textures::Textures, util::{Vector2}
};

use ggez::{
    Context,
    graphics::{self, DrawParam, Rect, WrapMode},
};

use crate::{
    obj::{body::Body},
};

// use std::path::Path;
// use std::fs::File;
// use std::io::{Write, BufRead, BufReader};

// use bincode;

mod material;
pub use material::*;

#[derive(Debug, Clone)]
pub struct World {
    pub palette: Palette,
    pub static_shapes: Vec<(Body, String)>,
    pub dynamic_shapes: Vec<(Body, String)>,
    // pub exit: Option<Point2>,
    // pub exit: Option<Point2>,
    // pub intels: Vec<Point2>,
}

impl World {
    pub fn draw_mat(&self, ctx: &mut Context, textures: &Textures) -> GgezResult<()> {
        for (body, image) in &self.static_shapes {
            let mut img = textures.get_img(ctx, &image).clone();
            img.set_wrap(WrapMode::Tile, WrapMode::Tile);
            let scale = Vector2::new(
                body.shape.width()/img.dimensions().w, 
                body.shape.height()/img.dimensions().h);
            let drawparams = DrawParam {
                src: Rect::new(0., 0., scale.x, scale.y),
                dest: body.shape.get_pos().into(),
                rotation: body.shape.rot,
                color: graphics::WHITE,
                .. Default::default()
            };
            
            graphics::draw(ctx, &img, drawparams)?;
            // let rect = Rect::new(0.,0., shape.width(), shape.height());
            // let mesh = MeshBuilder::new()
            //     .rectangle(DrawMode::fill(), rect, graphics::WHITE)
            //     .texture(img.clone())
            //     .build(ctx)?;
            // graphics::draw(ctx, &mesh, drawparams)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub palette: Palette,
    pub static_shapes: Vec<(Body, String)>,
    pub dynamic_shapes: Vec<(Body, String)>,
    // pub start_point: Option<Point2>,
    // pub exit: Option<Point2>,
    // pub intels: Vec<Point2>,
}

impl Level {
    pub fn new(palette: Palette) -> Self {//, width: u16, height: u16) -> Self {
    // pub fn new() -> Self {
        Self {
            palette,
            static_shapes: Vec::new(),
            dynamic_shapes: Vec::new(),
            // start_point: None,
            // exit: None,
            // intels: Vec::new(),
        }
    }
    pub fn new_default() -> Self {//, width: u16, height: u16) -> Self {
        Self {
            palette: Palette::default(),
            static_shapes: Vec::new(),
            dynamic_shapes: Vec::new(),
            // start_point: None,
            // exit: None,
            // intels: Vec::new(),
        }
    }
    pub fn add_static_shape(&mut self, b: Body, s: String) {
        self.static_shapes.push((b, s));
    }
    pub fn add_dynamic_shape(&mut self, b: Body, s: String) {
        self.dynamic_shapes.push((b, s));
    }
}