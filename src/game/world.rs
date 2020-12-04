use kondi::{
    GgezResult, textures::Textures, util::{Vector2}, object::Object, State,
};

use ggez::{
    Context,
    error::GameError,
    graphics::{self, DrawParam, Rect, WrapMode},
};

use crate::{
    obj::{body::Body},
};

use std::path::Path;
use std::fs::File;
use std::io::{Write, BufRead, BufReader};

use bincode;

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
}

impl Object for World {
    fn draw(&self, ctx: &mut Context, texes: &Textures) -> GgezResult<()> { 
        for (body, image) in &self.static_shapes {
            let mut img = texes.get_img(ctx, &image).clone();
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
        }
        Ok(())
    }
    fn update(&mut self, _ctx: &mut Context, _state: &mut State, _delta: f32) {
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
    pub fn load<P: AsRef<Path>>(path: P) -> GgezResult<Self> {
        let mut reader = BufReader::new(File::open(path)?);
        let mut ret = Level::new_default();
        loop {
            let mut buf = String::with_capacity(16);
            reader.read_line(&mut buf)?;
            match &*buf.trim_end() {
                "" => continue,
                "STATIC_SHAPES" => ret.static_shapes = bincode::deserialize_from(&mut reader)
                    .map_err(|e| GameError::ResourceLoadError(format!("{:?}", e)))?,
                "DYNAMIC_SHAPES" => ret.dynamic_shapes = bincode::deserialize_from(&mut reader)
                    .map_err(|e| GameError::ResourceLoadError(format!("{:?}", e)))?,
                "END" => break, 
                _ => return Err(GameError::ResourceLoadError("Bad section".to_string()))
            }
        }
        Ok(ret)
    }
    pub fn save<P: AsRef<Path>>(&self, path: P) -> GgezResult<()> {
        let mut file = File::create(path)?;
        writeln!(file, "STATIC_SHAPES")?;
        bincode::serialize_into(&mut file, &self.static_shapes)
            .map_err(|e| GameError::ResourceLoadError(format!("{:?}", e)))?;
        writeln!(file, "DYNAMIC_SHAPES")?;
        bincode::serialize_into(&mut file, &self.dynamic_shapes)
            .map_err(|e| GameError::ResourceLoadError(format!("{:?}", e)))?;
        writeln!(file, "\nEND")?;
        Ok(())
    }
    // Serialize, Deserialize
}