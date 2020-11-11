use kondi::{
    Context, Game, GgezResult,
    textures::Textures,
    util::{Point2, Vector2},
};
use ggez::{graphics::{self, Image, spritebatch, Rect}};
use crate::{
    utils::spritesheet::SpriteSheet,
};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
use std::fs::File;
use std::io::Read;
use std::cell::Ref;

lazy_static! {
    static ref MATS: RwLock<HashMap<String, Mat>> = {
        RwLock::new(HashMap::with_capacity(10))
    };
}

fn ensure(mat: &str) {
    if !MATS.read().unwrap().contains_key(mat) {
        let props = if let Ok(mut f) = File::open(format!("resources/materials/{}.mat", mat)) {
            let mut s = String::new();
            f.read_to_string(&mut s).unwrap();

            toml::from_str(&s).unwrap()
        } else {
            MaterialProperties::default()
        };
        let mat_data = Mat { spr: (format!("materials/{}", mat)), props};

        MATS.write().unwrap().insert(mat.to_owned(), mat_data);
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct MaterialProperties {
    solid: bool,
}

#[inline]
pub fn is_solid(mat: &str) -> bool {
    ensure(mat);

    MATS.read().unwrap()[mat].props.solid
}

#[inline]
pub fn get_img<'a>(ctx: &mut Context, textures: &'a Textures, mat: &str) -> Ref<'a, Image> {
    ensure(mat);

    textures.get_img(ctx, &MATS.read().unwrap()[mat].spr)
}

pub struct Mat {
    spr: String,
    props: MaterialProperties,
}

#[derive(Debug, Clone)]
pub struct Palette {
    pub block_materials: SpriteSheet,
}

impl Default for Palette {
    fn default() -> Self {
        let names = vec![
            "white_brick",
            "white_brick_damaged",
            "grey_brick",
            "grey_brick_damaged",
            "concrete_brick",
            "concrete",
            "metal_box_damaged",
            "metal_box",
            "warning",
            "grass",
            "dirt",
            "bricks",
        ];
        let img_dims = Rect::new(0., 0., 192., 128.);
        let mut mats = Vec::new();
        let mut c_w = 0.;
        let mut c_h = 0.;
        for _name in &names {
            let mut temp = Vec::new();
            temp.push(
                Rect::new(
                    32.*c_w, 
                    32.*c_h, 
                    32.,
                    32.,
                ));
            c_w += 1.;
            mats.push(temp);
            if (c_w*32.) >= img_dims.w {
                c_h += 1.;
                c_w = 0.;
            }
        }
        Palette {
            block_materials: SpriteSheet::new("materials/materials_spritesheet", img_dims, names, mats),
        }
    }
}

impl Palette {
    pub fn new(ctx: &mut Context, textures: &Textures, texture: &str, names: Vec<&str>) -> Self {
        let img_dims = textures.get_img(ctx, texture).dimensions();
        let mut mats = Vec::new();
        let mut c_w = 0.;
        let mut c_h = 0.;
        for _name in &names {
            let mut temp = Vec::new();
            temp.push(
                Rect::new(
                    32.*c_w,
                    32.*c_h,
                    32.,
                    32.,
                ));
            c_w += 1.;
            mats.push(temp);
            if (c_w*32.) >= img_dims.w as f32 {
                c_h += 1.;
                c_w = 0.;
            }
        }
        Palette {
            block_materials: SpriteSheet::new(texture, img_dims, names, mats),
        }
    }
    pub fn draw_mat(&self, ctx: &mut Context, mesh, name: String) -> GgezResult<()> {
        // graphics::draw(ctx, &self.world_batch, dp)
        // for (shape, name) in &self.static_shapes {
        graphics::draw(ctx, , self.block_materials.spriteparts.get(sprite).unwrap()[0].dest(shape.pos))
        // }
    }
    // pub fn draw_mat(&self, i: u8, ctx: &mut Context, textures: &Textures, x: f32, y: f32, dp: graphics::DrawParam) -> GgezResult<()> {
    //     let some(mat) = self.materials[i as usize];

    //     let img = get_img(ctx, assets, mat);
    //     graphics::draw(ctx, &*img, (Point2::from(dp.dest) + Vector2::new(x, y),))
    // }
    // pub fn is_solid(&self, i: u8) -> bool {
    //     is_solid(self.materials[i as usize])
    // }
    // #[inline]
    // pub fn get(&self, i: u8) -> Option<&str> {
    //     self.materials.get(i as usize).copied()
    // }
    // #[inline]
    // pub fn find(&self, mat: &str) -> Option<u8> {
    //     self.materials.iter().position(|s| &mat == s).map(|i| i as u8)
    // }
    // #[inline]
    // pub fn len(&self) -> usize {
    //     self.materials.len()
    // }
    // #[inline]
    // pub fn is_empty(&self) -> bool {
    //     self.materials.is_empty()
    // }
}