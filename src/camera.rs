use vec3::Vec3;

use crate::render::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = crate::util::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).normalize();
        let u = Vec3::cross(&vup, &w).normalize();
        let v = Vec3::cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;


        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
        }
    }

    /// Get a ray from the camera to the given position on the viewport.
    /// s, t are in the range of [0, 1].
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            position: self.origin,
            velocity: (self.lower_left_corner
                + self.horizontal * s + &self.vertical * t
                - &self.origin).normalize(),
        }
    }
}
