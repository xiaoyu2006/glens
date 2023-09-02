use vec3::Vec3;

use crate::pic::Color;

pub trait Hittable {
    fn intersect(&self, p1: Vec3, p2: Vec3) -> Option<Color>;
}
