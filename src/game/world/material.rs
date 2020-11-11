use kondi::{
    Context, Game, GgezResult,
    textures::Textures,
    util::{Point2, Vector2},
};
use ggez::{graphics::{self, Image, DrawParam, spritebatch::{SpriteBatch, SpriteIdx}, Rect}};
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
    materials: HashMap<String, DrawParam>,
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
        let mut mats = HashMap::new();
        let mut c_w = 0.;
        let mut c_h = 0.;
        for name in names {
            mats.insert(name.to_string(), DrawParam::default().src(
                SpriteSheet::to_size(Rect::new(
                    0.,
                    0.,
                    192.,
                    128.,
                ),
                    Rect::new(
                        32.*c_w, 
                        32.*c_h, 
                        32.,
                        32.,
                    )
                )));
            c_w += 1.;
            if (c_w*32.) > 192. {
                c_h += 1.;
            }
        }
        Palette {
            materials: mats,
        }
    }
}

impl Palette {
    pub fn new(names: Vec<&str>) -> Self {
        let mut mats = HashMap::new();
        // let img = t.get_img(ctx, texture);
        let dp = DrawParam::default();
        let w = 192.;
        let h = ((names.len() as f32)*32./w).ceil();
        let mut c_w = 0.;
        let mut c_h = 0.;
        for name in names {
            mats.insert(name.to_string(), DrawParam::default().src(
                SpriteSheet::to_size(
                    Rect::new(0.,0., w, h),
                    Rect::new(
                        32.*c_w,
                        32.*c_h,
                        32.,
                        32.,
                    )
                )));
            c_w += 1.;
            if (c_w*32.) > w {
                c_h += 1.;
            }
        }
        Palette {
            materials: mats,
        }
    }
    pub fn and(self, other: &Self) -> Self {
        let Palette{materials} = self;
        let mut mats = materials.to_vec();
        
        for &mat in &*other.materials {
            if !mats.contains(&mat) {
                mats.push(mat);
            }
        }

        Palette {
            materials: mats.into_boxed_slice(),
        }
    }
    pub fn draw_mat(&self, i: u8, ctx: &mut Context, textures: &Textures, x: f32, y: f32, dp: graphics::DrawParam) -> GgezResult<()> {
        let mat = self.materials[i as usize];

        let img = get_img(ctx, assets, mat);
        graphics::draw(ctx, &*img, (Point2::from(dp.dest) + Vector2::new(x, y),))
    }
    pub fn is_solid(&self, i: u8) -> bool {
        is_solid(self.materials[i as usize])
    }
    #[inline]
    pub fn get(&self, i: u8) -> Option<&str> {
        self.materials.get(i as usize).copied()
    }
    #[inline]
    pub fn find(&self, mat: &str) -> Option<u8> {
        self.materials.iter().position(|s| &mat == s).map(|i| i as u8)
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.materials.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.materials.is_empty()
    }

}