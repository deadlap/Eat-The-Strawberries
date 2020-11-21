use kondi::{textures::Textures, Context, GgezResult, util::{Point2, Vector2}};

use std::collections::HashMap;
use ggez::{graphics::{self, Rect, DrawParam}};

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub spritesheet: String,
    // pub src_size: Vector2,
    pub spriteparts: HashMap<String, Vec<(DrawParam, Vector2)>>,
}

impl SpriteSheet {
    // pub fn new(texture: &str, img_dims: Rect, src_size: Vector2, names: Vec<&str>, sprites: Vec<Vec<Rect>>) -> Self {
    pub fn new(texture: &str, img_dims: Rect, names: Vec<&str>, sprites: Vec<Vec<Rect>>) -> Self {
        let mut sprite_parts = HashMap::new();
        let dp = DrawParam::default();
        let mut c = 0;
        for name in names {
            let mut temp: Vec<(DrawParam, Vector2)> = Vec::new();
            for rect in &*sprites[c] {
                temp.push((dp.src(SpriteSheet::to_size(img_dims, *rect)), Vector2::new(rect.w, rect.h)));
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
        let img = t.get_img(ctx, &self.spritesheet).clone();
        graphics::draw(ctx, &img,
            (self.spriteparts.get(&part_name.to_string()).unwrap()[index].0).dest(dest))
    }
    pub fn draw_with_offset(&self, ctx: &mut Context, t: &Textures, part_name: &str, index: usize, dest: Point2, src_size: Vector2) -> GgezResult<()> {
        let img = t.get_img(ctx, &self.spritesheet).clone();
        let src = self.spriteparts.get(&part_name.to_string()).unwrap()[index];
        let new_dest = dest - (src.1 - src_size);
        graphics::draw(ctx, &img, (src.0).dest(new_dest))
    }
    // pub fn change_Param(&mut self, ctx: &mut Context, part_name: &str, index: usize) {
        
    // }
}