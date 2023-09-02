#[cfg(test)]
mod test {
    use vec3::Vec3;

    use crate::{render::VisibleObject, pic::Color};

    #[test]
    fn test_sphere_intersect() {
        let sph = VisibleObject {
            position: Vec3::new(0.0, 0.0, 0.0),
            color: Color(0.0, 0.0, 0.0),  // not used
            radius: 1.0,
        };
        assert!(sph.intersect_sphere(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0)));
        assert!(sph.intersect_sphere(Vec3::new(0.0, 0.0, 0.0), Vec3::new(2.0, 0.0, 0.0)));
        assert!(sph.intersect_sphere(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0)));
        assert!(!sph.intersect_sphere(Vec3::new(1.0, 1.0, 1.0), Vec3::new(2.0, 1.0, 1.0)))
    }
}
