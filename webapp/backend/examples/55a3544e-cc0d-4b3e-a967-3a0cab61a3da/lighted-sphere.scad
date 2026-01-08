$fa = 1;
$fs = 0.4;
$fn = 100;

include <caustic.scad>

// camera
camera(
  aspect_ratio=16.0 / 9.0,
  image_width=400,
  samples_per_pixel=50,
  max_depth=50,
  defocus_angle=0,
  vertical_fov=20,
  look_from=[-26, 6, 3],
  look_at=[0, 0, 2],
  background=[0, 0, 0]
);

// ground
lambertian(t=perlin_turbulence(scale=4, turbulence_depth=7))
  translate([0, 0, -1000])
    sphere(r=1000);

lambertian(t=perlin_turbulence(scale=4, turbulence_depth=7))
  translate([0, 0, 2])
    sphere(r=2);

diffuse_light(c=[4, 4, 4])
  quad(q=[-3, -2, 1], u=[-2, 0, 0], v=[0, 0, 2]);

diffuse_light(c=[0, 0, 2])
  translate([0, 0, 7])
    sphere(r=2);
