use kondi::{textures::Textures, Context, GgezResult, util::Point2};

use std::collections::HashMap;
use ggez::{graphics::{self, Rect, DrawParam, WrapMode}};

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    // pub spritesheet: Image,
    pub spritesheet: String,
    pub spriteparts: HashMap<String, Vec<DrawParam>>,
}

impl SpriteSheet {
    pub fn new(texture: &str, img_dims: Rect, names: Vec<&str>, sprites: Vec<Vec<Rect>>) -> Self {
        let mut sprite_parts = HashMap::new();
        let dp = DrawParam::default();
        let mut c = 0;
        for name in names {
            let mut temp: Vec<DrawParam> = Vec::new();
            for rect in &*sprites[c] {
                temp.push(dp.src(SpriteSheet::to_size(img_dims, *rect)));
            }
            sprite_parts.insert(name.to_string(), temp);
            c += 1;
        }
        SpriteSheet {
            spritesheet: texture.to_string(),
            spriteparts: sprite_parts,
        }
    }
    pub fn to_size(img_rect: Rect, cur_rect: Rect) -> Rect {
        Rect::new(
            cur_rect.x/img_rect.w,
            cur_rect.y/img_rect.h,
            cur_rect.w/img_rect.w,
            cur_rect.h/img_rect.h,
        )
    }
    pub fn draw(&self, ctx: &mut Context, t: &Textures, part_name: &str, index: usize, dest: Point2) -> GgezResult<()> {
        let mut img = t.get_img(ctx, &self.spritesheet).clone();
        img.set_wrap(WrapMode::Tile, WrapMode::Tile);
        graphics::draw(ctx,
            &img,
            (self.spriteparts.get(&part_name.to_string()).unwrap()[index]).dest(dest),
        )
    }
    // pub fn change_Param(&mut self, ctx: &mut Context, part_name: &str, index: usize) {
        
    // }
}