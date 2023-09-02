use rayon::prelude::*;

use crate::render::World;

#[derive(Copy, Clone, Debug)]
pub struct Color(pub f32, pub f32, pub f32);

pub struct Picture {
    pub pixels: Vec<Color>,
    pub dimensions: (usize, usize),
}

impl Picture {
    pub fn new(width: usize, height: usize, world: &World) -> Self {
        let pixels = (0..width * height)
            .into_par_iter()
            .map(|i| {
                let x = i % width;
                let y = i / width;
                world.render_pixel(x, y)
            })
            .collect();
        Picture {
            pixels,
            dimensions: (width, height),
        }
    }

    pub fn write(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("P3\n{} {}\n255\n", self.dimensions.0, self.dimensions.1));
        for pixel in &self.pixels {
            s.push_str(&format!(
                "{} {} {}\n",
                (pixel.0 * 255.0) as u8,
                (pixel.1 * 255.0) as u8,
                (pixel.2 * 255.0) as u8
            ));
        }
        s
    }
}
