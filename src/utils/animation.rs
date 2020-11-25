use kondi::{State, Context};
use crate::DELTA;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AnimationContainer {
    pub animation_time: f32,
    pub anim_timers: HashMap<String, (f32, usize)>,
    pub cur_step: usize,
    pub cur_img: String,
}

impl AnimationContainer {
    // pub fn new(names: Vec<&str>, sprites: Vec<Vec<f32>> ) -> Self {
    pub fn new(names: Vec<&str>, timers: Vec<(f32, usize)> ) -> Self {
        let name1 = (names.clone())[0];
        let mut anim_timers = HashMap::new();
        let mut i = 0;
        for name in names {
            anim_timers.insert(name.to_string(), timers[i].clone());
            i += 1;
        }
        AnimationContainer {
            animation_time: 0.,
            anim_timers: anim_timers,
            cur_step: 0,
            cur_img: name1.to_string(),
        }
    }
    pub fn change_image(&mut self, to_image: &str) {
        self.cur_img = to_image.to_string();
        self.animation_time = 0.;
        self.cur_step = 0;
    }
    pub fn update_timer(&mut self) {
        self.animation_time += DELTA;
        if self.animation_time >= (self.cur_step as f32) * self.anim_timers.get(&self.cur_img).unwrap().0 {
            self.cur_step += 1;
        }
        if self.cur_step >= self.anim_timers.get(&self.cur_img).unwrap().1 {
            self.cur_step = 0;
            self.animation_time = 0.;
        }
    }
}