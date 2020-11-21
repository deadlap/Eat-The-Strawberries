use kondi::util::{Point2, Vector2};
use ggez::graphics::Rect;
use std::f32::consts::PI;
use crate::DELTA;
#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub pos: Point2, 
    pub radius: f32,
}

#[derive(Debug, Clone, Copy)]
// #[serde(rename_all = "lowercase")]
pub enum ShapeType{
    Rectangle {
        rect: Rect,
    },
    Circle {
        circle: Circle
    },
}

#[derive(Debug, Clone, Copy)]
pub struct Shape {
    pub rot: f32,
    pub shape: ShapeType,
}

impl Shape {
    pub fn new(rot: f32, shape: ShapeType) -> Self {
        Shape {
            rot,
            shape,
        }
    }
    
    pub fn move_by(&mut self, change: Vector2) {
        let act_change = change*DELTA;
        match &mut self.shape {
            ShapeType::Rectangle{rect} => {
                rect.translate(act_change);
            },
            ShapeType::Circle{circle} => {
                circle.pos.x += act_change.x;
                circle.pos.y += act_change.y;
            }
        }
    }
    
    pub fn set_pos(&mut self, change: Point2) {
        match &mut self.shape {
            ShapeType::Rectangle{rect} => {
                rect.move_to(change);
            },
            ShapeType::Circle{circle} => {
                circle.pos.x = change.x;
                circle.pos.y = change.y;
            }
        }
    }
    
    pub fn rotate(&mut self, change: f32) {
        self.rot += change*DELTA;
    }
    
    pub fn get_pos(&self) -> Point2 {
        match &self.shape {
            ShapeType::Rectangle{rect} => {
                Point2::new(rect.x, rect.y)
            },
            ShapeType::Circle{circle} => {
                circle.pos
            }
        }
    }
    pub fn get_center(&self) -> Point2 {
        match &self.shape {
            ShapeType::Rectangle{rect} => {
                Point2::new(rect.x+rect.w/2., rect.y+rect.h/2.)
            },
            ShapeType::Circle{circle} => {
                circle.pos
            }
        }
    }
    pub fn dimensions(&self) -> Vector2 {
        match &self.shape {
            ShapeType::Rectangle{rect} => {
                Vector2::new(rect.w, rect.h)
            },
            ShapeType::Circle{circle} => {
                Vector2::new(circle.radius, circle.radius)
            }
        }
    }

    pub fn size(&self) -> f32 {
        match &self.shape {
            ShapeType::Rectangle{rect} => {
                rect.w*rect.h
            },
            ShapeType::Circle{circle} => {
                PI*(circle.radius).powi(2)
            }
        }
    }

    pub fn width(&self) -> f32 {
        match &self.shape {
            ShapeType::Rectangle{rect} => {
                rect.w
            },
            ShapeType::Circle{circle} => {
                circle.radius
            }
        }
    }

    pub fn height(&self) -> f32 {
        match &self.shape {
            ShapeType::Rectangle{rect} => {
                rect.h
            },
            ShapeType::Circle{circle} => {
                circle.radius
            }
        }
    }
}