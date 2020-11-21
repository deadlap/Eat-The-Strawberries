use kondi::util::{Vector2};
use crate::{
    obj::shape::Shape,
    utils::util,
};

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub density: f32,
    pub restitution: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct MassData {
    pub mass: f32,
    pub inv_mass: f32,

    pub inertia: f32,
    pub inv_inertia: f32,
}

impl MassData {
    pub fn new(mass: f32, inertia: f32) -> Self {
        MassData {
            mass: mass,
            inv_mass: MassData::inverse(mass),

            inertia: inertia,
            inv_inertia: MassData::inverse(inertia),
        }
    }
    fn inverse(item: f32) -> f32 {
        if item == 0. {
            0.
        } else {
            1./item
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub shape: Shape,
    pub material: Material,
    pub mass_data: MassData,
    pub velocity: Vector2,
    pub force: Vector2,
    pub gravity_scale: f32,

    // pub acceleration: f32,
    // pub angular_velocity: f32,
    // pub torque: f32,
}

impl Body {
    pub fn new(shape: Shape, mat: Material, scale: f32) -> Self {
        Body {
            shape: shape,
            material: mat,
            mass_data: MassData::new(mat.density*shape.size(), 0.),
            velocity: Vector2::new(0.,0.,),
            force: Vector2::new(0.,0.),
            gravity_scale: scale,
        }
    }
    pub fn with_mass(shape: Shape, mat: Material, scale: f32, mass: f32, inertia: f32) -> Self {
        Body {
            shape: shape,
            material: mat,
            mass_data: MassData::new(mass, inertia),
            velocity: Vector2::new(0.,0.,),
            force: Vector2::new(0.,0.),
            gravity_scale: scale,
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.add_vel((self.force*self.mass_data.inv_mass + Vector2::new(0., util::GRAVITY) * self.gravity_scale)*delta);
        self.shape.move_by(self.velocity);
        // if self.force.y.abs() > 0. {
        //     self.add_force(Vector2::new(0.,delta*100.));
        // } else if self.force.y.abs() <= delta {
        //     self.force = Vector2::new(self.force.x,0.,);
        // }
    }
    pub fn add_force(&mut self, change: Vector2) {
        self.force += change;
    }
    pub fn add_vel(&mut self, change: Vector2) {
        self.velocity += change;
    }
}