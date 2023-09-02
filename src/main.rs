pub mod options;
pub mod pic;
pub mod render;
pub mod camera;
pub mod util;
pub mod hittable;
pub mod object;

use pic::Picture;
use structopt::StructOpt;

fn main() {
    let options = options::Options::from_args();
    let world = options.to_world();
    let pic = Picture::new(options.width, options.height, &world);
    if let Some(output) = options.output {
        std::fs::write(output, pic.write()).expect("Unable to write file");
    } else {
        println!("{}", pic.write());
    }
}
