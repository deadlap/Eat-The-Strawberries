use kondi::{textures::Textures, Context, GgezResult, util::{Point2, Vector2}};

use std::collections::HashMap;
use ggez::{graphics::{self, Rect, DrawParam}};
use crate::{utils::util::Orientation};

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub spritesheet: String,
    pub orientation: Orientation,
    pub spriteparts: HashMap<String, Vec<(DrawParam, Vector2)>>,
}

impl SpriteSheet {
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
            orientation: Orientation::Right
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
        let dp = self.spriteparts.get(&part_name.to_string()).unwrap()[index].0.clone();
        let scale: Vector2;
        if self.orientation == Orientation::Left {
            scale = Vector2::new(-1.,1.);
        } else {
            scale = Vector2::new(1.,1.);
        }
        graphics::draw(ctx, &img, dp.dest(dest).scale(scale))
    }
    pub fn draw_with_offset(&self, ctx: &mut Context, t: &Textures, part_name: &str, index: usize, dest: Point2, src_size: Vector2) -> GgezResult<()> {
        let img = t.get_img(ctx, &self.spritesheet).clone();
        let src = self.spriteparts.get(&part_name.to_string()).unwrap()[index];
        let dp = src.0;
        let scale: Vector2; 
        let new_dest: Point2;
        if self.orientation == Orientation::Left {
            new_dest = dest - (src.1 - src_size) + Vector2::new(src.1.x+src.1.x-src_size.x,0.);
            scale = Vector2::new(-1.,1.);
        } else {
            scale = Vector2::new(1.,1.);
            new_dest = dest - (src.1 - src_size);
        }
        graphics::draw(ctx, &img, dp.dest(new_dest).scale(scale))
    }

    pub fn change_orientation(&mut self, orientation: Orientation) {
        self.orientation = orientation;
    }
    pub fn switch_orientation(&mut self) {
        if self.orientation == Orientation::Left {
            self.orientation = Orientation::Right;
        } else {
            self.orientation = Orientation::Left;
        }
    }
    // pub fn change_Param(&mut self, ctx: &mut Context, part_name: &str, index: usize) {
        
    // }
}