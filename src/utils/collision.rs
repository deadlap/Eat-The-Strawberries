use kondi::util::Vector2;
use ggez::graphics::Rect;
use nalgebra::distance;
use crate::{
    obj::{shape::{Circle, ShapeType},
        body::Body},
    DELTA,
};

const CORRECTION_PERCENT: f32 = 1.0;
const SLOP: f32 = 0.01;

pub struct CollisionData<'a> {
    pub object_a: &'a mut Body,
    pub object_b: &'a mut Body,
    pub penetration: f32,
    pub normal: Vector2,
}

impl<'a> CollisionData<'a> {
    pub fn check_collision(body_a: &'a mut Body, body_b: &'a mut Body) -> Option<CollisionData<'a>> {
        let shape_a = body_a.shape;
        let shape_b = body_b.shape;
        let mut data = CollisionData::<'a> {
            object_a: body_a,
            object_b: body_b,
            penetration:  0.,
            normal: Vector2::new(0.,0.),
        };
        let collision_happened = match (shape_a.shape, shape_b.shape) {
            (ShapeType::Rectangle{rect: rect_a}, ShapeType::Rectangle{rect: rect_b}) =>
                (&mut data).rect_vs_rect(rect_a, rect_b),
            (ShapeType::Circle{circle: circle_a}, ShapeType::Circle{circle: circle_b}) =>
                (&mut data).circle_vs_circle(circle_a, circle_b),
            (ShapeType::Circle{circle}, ShapeType::Rectangle{rect}) | 
                (ShapeType::Rectangle{rect}, ShapeType::Circle{circle}) =>
                    (&mut data).circle_vs_rect(circle, rect),
        };
        if !collision_happened {
            return None;
        } else {
            Some(data)
        }
    }

    pub fn resolve_collision(&mut self) {
        let ref mut body_a = self.object_a;
        let ref mut body_b = self.object_b;

        let relative_vel = body_b.velocity-body_a.velocity;
        let vel_along_norm = relative_vel.dot(&self.normal);

        if vel_along_norm > 0. {
            return
        }
        
        let rest = body_a.material.restitution.min(body_b.material.restitution);
        
        let mut impulse_scalar = -(1.+rest) * vel_along_norm;
        impulse_scalar /= body_a.mass_data.inv_mass + body_b.mass_data.inv_mass;
        
        let impulse = impulse_scalar*self.normal;
        body_a.add_vel(-1.*(body_a.mass_data.inv_mass*impulse));
        body_b.add_vel(body_b.mass_data.inv_mass*impulse);
    }

    pub fn positional_correction(&mut self) {
        let body_a = self.object_a.clone();
        let body_b = self.object_b.clone();

        let correction = (0.0f32.max(self.penetration - SLOP) / 
            (body_a.mass_data.inv_mass+body_b.mass_data.inv_mass))*CORRECTION_PERCENT*self.normal;
        
        self.object_a.shape.move_by(-1.*((body_a.mass_data.inv_mass * correction)/DELTA));
        self.object_b.shape.move_by((body_b.mass_data.inv_mass * correction)/DELTA);
    }

    // Functions to check for collisions between shapes.
    fn rect_vs_rect(&mut self, rect_a: Rect, rect_b: Rect) -> bool {
        if !rect_a.overlaps(&rect_b) {
            return false;
        }
        let a_pos = self.object_a.shape.get_center();
        let b_pos = self.object_b.shape.get_center();
        let norm: Vector2 = b_pos-a_pos;

        let ax_extent = rect_a.w/2.;
        let bx_extent = rect_b.w/2.;
        let x_overlap = ax_extent+bx_extent - norm.x.abs();
        if x_overlap > 0. {
            let ay_extent = rect_a.h/2.;
            let by_extent = rect_b.h/2.;
            let y_overlap = ay_extent+by_extent - norm.y.abs();
            
            if y_overlap > 0. {
                if x_overlap < y_overlap {
                    if norm.x < 0. {
                        self.normal = Vector2::new(-1.,0.);
                    } else {
                        self.normal = Vector2::new(1.,0.);
                    }
                    self.penetration = x_overlap;
                } else {
                    if norm.y < 0. {
                        self.normal = Vector2::new(0.,-1.);
                    } else {
                        self.normal = Vector2::new(0.,1.);
                    }
                    self.penetration = y_overlap;
                }
                return true;
            }
        }
        false
    }
    fn circle_vs_circle(&mut self, circle_a: Circle, circle_b: Circle) -> bool {
        let norm: Vector2 = circle_b.pos - circle_a.pos;
        let dist = distance(&circle_a.pos, &circle_b.pos);
        let rad = circle_a.radius + circle_b.radius;

        if dist > rad.powi(2) {
            return false;
        }

        if dist != 0. {
            self.penetration = rad - dist;
            self.normal = norm/dist;
        } else {
            self.penetration = circle_a.radius;
            self.normal = Vector2::new(1.,0.);
        }
        true
    }
    fn circle_vs_rect(&mut self, circle: Circle, rect: Rect) -> bool {
        if rect.contains(circle.pos) {
            return true;
        }
        false
    }
}