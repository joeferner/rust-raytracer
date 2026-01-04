$fa = 1;
$fs = 0.4;
$fn = 100;

include <ray_trace.scad>

// camera
camera(
  aspect_ratio=16.0 / 9.0,
  image_width=300,
  samples_per_pixel=10,
  max_depth=50,
  vertical_fov=20,
  look_from=[0, 0, 12],
  look_at=[0, 0, 0],
  defocus_angle=0,
  background=[0.7, 0.8, 1.0]
);

// globe
lambertian(t=texture(filename="earth-map.jpg"))
  sphere(r=2.0);
