use vec3::Vec3;

use crate::{pic::Color, camera::Camera};

#[derive(Debug)]
pub struct MassPoint {
    pub position: Vec3,
    pub mass: f32,
}

#[derive(Debug)]
pub struct VisibleObject {
    pub position: Vec3,
    pub color: Color,
    pub radius: f32,
}

impl VisibleObject {
    pub fn inside_sphere(&self, p: Vec3) -> bool {
        (p - self.position).length_squared() < self.radius * self.radius
    }

    /// If the segment <p1,p2> intersects with the sphere.
    /// The segment is considered to intersect with the sphere if either of its endpoints is inside the sphere.
    /// There are edge cases where the segment is crossing the sphere but neither of its endpoints is inside the sphere.
    /// But this situation is omitted for simplicity since delta_t is small enough.
    pub fn intersect_sphere(&self, p1: Vec3, p2: Vec3) -> bool {
        self.inside_sphere(p1) || self.inside_sphere(p2)
    }
}


/// Although the name is `Ray`, it's actually a particle.
/// We somehow inverse-trace the proton from the camera to the object since light travels in the same manner as a particle with mass does.
#[derive(Debug)]
pub struct Ray {
    pub position: Vec3,
    /// Fake velocity, actually the direction of the ray.
    /// We don't need the time infomation of the ray so anyway.
    pub velocity: Vec3,
}

impl Ray {
    pub fn step(&mut self, delta: f32) -> (Vec3, Vec3) {
        let p1 = self.position;
        self.position += self.velocity * delta;
        (p1, self.position)
    }
}


/// Encapsulates configuration for the simulation.
#[derive(Debug)]
pub struct World {
    // =====SCENE=====
    pub mass_points: Vec<MassPoint>,
    pub visible_objects: Vec<VisibleObject>,

    // =====SIMULATION=====
    /// The boundaries of the simulation. Specifies a cuboid with the given corners.
    pub boundaries: (Vec3, Vec3),
    /// Maximum number of iterations to calculate for each ray.
    /// Either a ray reaches the boundaries or this number of iterations is reached will be discarded.
    pub max_iterations: usize,
    pub delta_t: f32,

    // =====CAMERA=====
    pub cam: Camera,

    // =====OUTPUT=====
    /// Width of the output image in pixels.
    pub width: usize,
    /// Height of the output image in pixels.
    pub height: usize,
}

impl World {
    pub fn render_pixel(&self, px_width: usize, px_height: usize) -> Color {
        let u = (px_width as f32 + 0.5) / self.width as f32;
        let v = (px_height as f32 + 0.5) / self.height as f32;
        let mut ray = self.cam.get_ray(u, v);

        for _it in 0..self.max_iterations {
            let acceleration = self.get_acceleration(ray.position);
            ray.velocity += acceleration * self.delta_t;

            let (p1, p2) = ray.step(self.delta_t);
            if self.is_out_of_boundaries(p2) {
                return Color(0.0, 0.0, 0.0);
            }
            if let Some(color) = self.intersect(p1, p2) {
                return color;
            }
        }

        Color(0.0, 0.0, 0.0)
    }

    fn get_acceleration(&self, position: Vec3) -> Vec3 {
        let mut acceleration = Vec3::new(0.0, 0.0, 0.0);
        for mass_point in &self.mass_points {
            let direction = mass_point.position - position;
            // println!("mass_point.position: {:?}", mass_point.position);
            // println!("position: {:?}", position);
            // println!("direction: {:?}", direction);
            let mut distance_squared = direction.length_squared();
            if distance_squared < 1e-2 {
                distance_squared = 1e-2;  // avoid division by zero
            }
            let accl_mag = mass_point.mass / distance_squared;
            acceleration += direction.normalize() * accl_mag;
        }
        acceleration
    }

    fn is_out_of_boundaries(&self, p: Vec3) -> bool {
        let (min, max) = self.boundaries;
        p.x < min.x || p.y < min.y || p.z < min.z || p.x > max.x || p.y > max.y || p.z > max.z
    }

    /// If the segment <p1,p2> intersects with any object, return the object and the distance to the intersection.
    fn intersect(&self, p1: Vec3, p2: Vec3) -> Option<Color> {
        for obj in &self.visible_objects {
            if obj.intersect_sphere(p1, p2) {
                return Some(obj.color);
            }
        }
        None
    }
}
