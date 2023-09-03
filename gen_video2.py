#!/usr/bin/env python3

import subprocess, os
from math import sin, cos, pi

CONFIG_TEMPLATE = [
    "--look-at=0,2,0",
    "--look-from=0,0,-0.05",
    "--up=0,0,1",
    "--delta-t=0.01",
    "--iterations=2000",
    "--spheres=",
    "--focus=60",
]

BH_MOTION_CENTER = (0, 2, 0)

if not os.path.exists("output2"):
    os.makedirs("output2")

class Blackhole:
    def __init__(
        self,
        mass: float,
        pos: tuple[float, float, float],
        color: tuple[float, float, float],
        disk_radius: float,
    ) -> None:
        self.mass = mass
        self.pos = pos
        self.col = color
        self.radius = disk_radius

    def set_pos(self, new_pos) -> None:
        self.pos = new_pos


def bhs_to_args(bhs: list[Blackhole]) -> list[str]:
    mass_points = "--mass-points="
    mass_points += ";".join(
        [f"{bh.pos[0]:.4f},{bh.pos[1]:.4f},{bh.pos[2]:.4f},{bh.mass:.4f}" for bh in bhs]
    )
    disks = "--disks="
    disks += ";".join(
        [
            f"{bh.pos[0]:.4f},{bh.pos[1]:.4f},{bh.pos[2]:.4f},{bh.col[0]:.4f},{bh.col[1]:.4f},{bh.col[2]:.4f},{bh.radius:.4f},0,0,1"
            for bh in bhs
        ]
    )
    return [mass_points, disks]


bh1 = Blackhole(0.2, (0, 3, 0), (0.96, 0.64, 0.25), 0.2)
bh2 = Blackhole(0.5, (0, 1, 0), (0.33, 0.26, 0.96), 0.4)

i = 0

for angle in range(0, 360, 2):
    bh1.set_pos(
        (
            BH_MOTION_CENTER[0] + cos(angle * pi / 180) * 0.6,
            BH_MOTION_CENTER[1] + sin(angle * pi / 180) * 0.6,
            BH_MOTION_CENTER[2],
        )
    )
    bh2.set_pos(
        (
            BH_MOTION_CENTER[0] + cos((angle + 180) * pi / 180) * 0.6,
            BH_MOTION_CENTER[1] + sin((angle + 180) * pi / 180) * 0.6,
            BH_MOTION_CENTER[2],
        )
    )
    args = (
        CONFIG_TEMPLATE + bhs_to_args([bh1, bh2]) + [f"--output-file=output2/{i}.ppm"]
    )
    cmd = ["./target/release/glens"] + args
    print(cmd)
    subprocess.run(cmd)
    i += 1

os.system("ffmpeg -framerate 15 -i output2/%d.ppm -vcodec libx264 -b 5000k -y output2/movie.mp4")
