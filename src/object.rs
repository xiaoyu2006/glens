use vec3::Vec3;

use crate::{hittable::Hittable, pic::Color};

#[derive(Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub color: Color,
    pub radius: f32,
}

impl Sphere {
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

impl Hittable for Sphere {
    fn intersect(&self, p1: Vec3, p2: Vec3) -> Option<Color> {
        if self.intersect_sphere(p1, p2) {
            Some(self.color)
        } else {
            None
        }
    }
}

pub struct Disk {
    pub position: Vec3,
    pub color: Color,
    pub radius: f32,
    pub normal: Vec3,
}

impl Disk {
    /// If the segment <p1,p2> intersects with the disk.
    pub fn intersect_disk(&self, p1: Vec3, p2: Vec3) -> bool {
        let v = p2 - p1;
        let t = Vec3::dot(&(self.position - p1), &self.normal) / Vec3::dot(&v, &self.normal);
        if t < 0.0 || t > 1.0 {
            return false;
        }
        let p = p1 + v * t;
        (p - self.position).length_squared() < self.radius * self.radius
    }
}

impl Hittable for Disk {
    fn intersect(&self, p1: Vec3, p2: Vec3) -> Option<Color> {
        if self.intersect_disk(p1, p2) {
            Some(self.color)
        } else {
            None
        }
    }
}
