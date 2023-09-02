use std::path::PathBuf;

use structopt::StructOpt;
use vec3::Vec3;

use crate::{render::{World, MassPoint, VisibleObject}, pic::Color, camera::Camera};

#[derive(Debug, StructOpt)]
#[structopt(name = "glens", about = "Gravitational lens simulator.")]
pub struct Options {
    // =====RENDERING=====

    /// Width of the output image in pixels.
    #[structopt(short = "w", long = "width", default_value = "960")]
    pub width: usize,

    /// Height of the output image in pixels.
    #[structopt(short = "h", long = "height", default_value = "540")]
    pub height: usize,

    /// Number of iterations to calculate for each pixel/ray.
    #[structopt(short = "i", long = "iterations", default_value = "5000")]
    pub iterations: usize,

    /// Delta t for each iteration.
    #[structopt(short = "d", long = "delta-t", default_value = "0.005")]
    pub delta_t: f32,

    /// Boundary of the simulation, in the form of "x1,y1,z1,x2,y2,z2".
    #[structopt(short = "b", long = "boundary", default_value = "-10,-10,-10,10,10,10")]
    pub boundary: String,

    /// =====CAMERA=====

    /// Where to look from, in the form of "x,y,z".
    #[structopt(short = "f", long = "look-from", default_value = "0,0,0")]
    pub camera_lookfrom: String,

    /// Where to look at, in the form of "x,y,z".
    #[structopt(short = "a", long = "look-at", default_value = "0,1,0")]
    pub camera_lookat: String,

    /// Camera field of view.
    #[structopt(short = "o", long = "focus", default_value = "80")]
    pub focus: f32,

    /// Camera-relative "up" direction, in the form of "x,y,z".
    #[structopt(short = "u", long = "up", default_value = "0,0,1")]
    pub camera_up: String,

    // =====SCENE=====

    /// Mass points in the world.
    /// Each mass point is in the form of "x,y,z,mass" and separated by a semicolon.
    /// They bend light but itself are not visible.
    #[structopt(short = "m", long = "mass-points", default_value = "0,2,0,1")]
    pub mass_points: String,

    /// Visible objects in the world.
    /// Each object is in the form of "x,y,z,r,g,b,radius" and separated by a semicolon.
    #[structopt(short = "v", long = "visible-objects", default_value = "0,6,0,2,1,0.8,0.6")]
    pub visible_objects: String,

    /// Gravitation constant.
    #[structopt(short = "g", long = "gravitation", default_value = "0.1")]
    pub gravity: f32,

    /// Output file, stdout if not present
    #[structopt(long = "output-file", parse(from_os_str))]
    pub output: Option<PathBuf>,
}

impl Options {
    pub fn to_world(&self) -> World {
        let mut mass_points = Vec::new();
        for mass_point in self.mass_points.split(';') {
            let mut parts = mass_point.split(',');
            let x = parts.next().unwrap().parse::<f32>().unwrap();
            let y = parts.next().unwrap().parse::<f32>().unwrap();
            let z = parts.next().unwrap().parse::<f32>().unwrap();
            let mass = parts.next().unwrap().parse::<f32>().unwrap();
            mass_points.push(MassPoint {
                position: Vec3::new(x, y, z),
                mass: mass * self.gravity,
            });
        }

        let mut visible_objects = Vec::new();
        for visible_object in self.visible_objects.split(';') {
            let mut parts = visible_object.split(',');
            let x = parts.next().unwrap().parse::<f32>().unwrap();
            let y = parts.next().unwrap().parse::<f32>().unwrap();
            let z = parts.next().unwrap().parse::<f32>().unwrap();
            let r = parts.next().unwrap().parse::<f32>().unwrap();
            let g = parts.next().unwrap().parse::<f32>().unwrap();
            let b = parts.next().unwrap().parse::<f32>().unwrap();
            let radius = parts.next().unwrap().parse::<f32>().unwrap();
            visible_objects.push(VisibleObject {
                position: Vec3::new(x, y, z),
                color: Color(r, g, b),
                radius,
            });
        }

        let mut parts = self.boundary.split(',');
        let x1 = parts.next().unwrap().parse::<f32>().unwrap();
        let y1 = parts.next().unwrap().parse::<f32>().unwrap();
        let z1 = parts.next().unwrap().parse::<f32>().unwrap();
        let x2 = parts.next().unwrap().parse::<f32>().unwrap();
        let y2 = parts.next().unwrap().parse::<f32>().unwrap();
        let z2 = parts.next().unwrap().parse::<f32>().unwrap();
        let boundaries = (Vec3::new(x1, y1, z1), Vec3::new(x2, y2, z2));

        let mut parts = self.camera_lookfrom.split(',');
        let x = parts.next().unwrap().parse::<f32>().unwrap();
        let y = parts.next().unwrap().parse::<f32>().unwrap();
        let z = parts.next().unwrap().parse::<f32>().unwrap();
        let camera_lookfrom = Vec3::new(x, y, z);

        let mut parts = self.camera_lookat.split(',');
        let x = parts.next().unwrap().parse::<f32>().unwrap();
        let y = parts.next().unwrap().parse::<f32>().unwrap();
        let z = parts.next().unwrap().parse::<f32>().unwrap();
        let camera_lookat = Vec3::new(x, y, z);

        let mut parts = self.camera_up.split(',');
        let x = parts.next().unwrap().parse::<f32>().unwrap();
        let y = parts.next().unwrap().parse::<f32>().unwrap();
        let z = parts.next().unwrap().parse::<f32>().unwrap();
        let camera_up = Vec3::new(x, y, z);

        World {
            mass_points,
            visible_objects,
            boundaries,
            max_iterations: self.iterations,
            delta_t: self.delta_t,
            cam: Camera::new(
                camera_lookfrom,
                camera_lookat,
                camera_up,
                self.focus,
                self.width as f32 / self.height as f32,
                1.0,
            ),
            width: self.width,
            height: self.height,
        }
    }
}

