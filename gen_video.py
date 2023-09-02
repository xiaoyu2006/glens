#!/usr/bin/env python3

import os

CONFIG_TEMPLATE = "\"--output-file=OUTPUT\" \"-v=0,6,0,2,1,0.8,0.6;4,8,0,0.6,0.6,0.9,1\" \"-m=POSX,2,0,1\" -i 2000"
START_X = -2
END_X = 2
STEP_X = 0.04

print("Generating video...")
print("Make sure you have `cargo build --release` and installed ffmpeg")

if not os.path.exists("output"):
    os.makedirs("output")

x = START_X
i = 0
while x <= END_X:
    x += STEP_X
    i += 1
    params = CONFIG_TEMPLATE.replace("POSX", str(x)).replace("OUTPUT", "output/" + str(i) + ".ppm")
    command = "./target/release/glens " + params
    print(command)
    os.system(command)

os.system("ffmpeg -framerate 10 -i output/%d.ppm -vcodec mpeg4 -y output/movie.mp4")
