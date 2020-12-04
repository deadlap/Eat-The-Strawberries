    
pub mod collision;
pub mod spritesheet;
pub mod animation;
pub mod save;

pub mod util {
    pub const GRAVITY: f32 = 64.;
    pub const UP: &'static str = "up";
    pub const DOWN: &'static str = "down";
    pub const LEFT: &'static str = "left";
    pub const RIGHT: &'static str = "right";
    pub const SPACE: &'static str = "space";
    pub const SHIFT: &'static str = "shift";
    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
    pub enum Orientation {
        Left,
        Right,
    }
    use kondi::util::Point2;
    use ggez::graphics::Rect;
    use crate::{
        obj::{shape::*, body::*},
        game::world::Level
    };

    pub fn create_example_level(w: f32, h: f32, pos: Point2) -> Level {
        let mut level = Level::new_default();

        let floor_rect = Rect::new(-w, pos.y, w*3., 32.);
        let floor_shape = Shape::new(0., ShapeType::Rectangle{rect: floor_rect});
        let floor_body = Body::with_mass(floor_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let roof_rect = Rect::new(0., 32., w, 32.);
        let roof_shape = Shape::new(0., ShapeType::Rectangle{rect: roof_rect});
        let roof_body = Body::with_mass(roof_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let left_rect = Rect::new(0., 64., 32., pos.y-128.);
        let left_shape = Shape::new(0., ShapeType::Rectangle{rect: left_rect});
        let left_wall = Body::with_mass(left_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);
        
        let right_rect = Rect::new(w-32., 64., 32., pos.y-88.);
        let right_shape = Shape::new(0., ShapeType::Rectangle{rect: right_rect});
        let right_wall = Body::with_mass(right_shape, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let platform_rect = Rect::new(32., pos.y-96., 128., 32.);
            let platform_shape = Shape::new(0., ShapeType::Rectangle{rect: platform_rect});
            let platform_body = Body::with_mass(platform_shape, 
                Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);

        let platform_rect2 = Rect::new(w-288., pos.y-64., 64., 64.);
        let platform_shape2 = Shape::new(0., ShapeType::Rectangle{rect: platform_rect2});
        let platform_body2 = Body::with_mass(platform_shape2, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);
        
        let platform_rect3 = Rect::new(224., pos.y-128., 32., 64.);
        let platform_shape3 = Shape::new(0., ShapeType::Rectangle{rect: platform_rect3});
        let platform_body3 = Body::with_mass(platform_shape3, 
            Material {density: 2., restitution: 0.1, static_friction: 0.3, dynamic_friction: 0.2}, 0., 0., 0.);
        
        level.add_static_shape(roof_body, "materials/brick_wall".to_string());
        level.add_static_shape(floor_body, "materials/concrete_wall".to_string());
        level.add_static_shape(left_wall, "materials/concrete_wall".to_string());
        level.add_static_shape(right_wall, "materials/concrete_wall".to_string());
        level.add_static_shape(platform_body, "materials/sidewalk".to_string());
        level.add_static_shape(platform_body2, "materials/brick_wall".to_string());
        level.add_static_shape(platform_body3, "materials/concrete_wall".to_string());

        return level
    }
}