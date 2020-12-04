use serde::{Serialize, Deserialize, Serializer, Deserializer};
use kondi::{util::{Vector2, Point2}};
use ggez::{graphics::{Rect}};

#[derive(Serialize, Deserialize)]
#[serde(remote = "Rect")]
pub struct RectDef {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}
impl From<RectDef> for Rect {
    fn from(def: RectDef) -> Self {
        Rect::new(def.x, def.y, def.w, def.h)
    }
}
