$fa = 1;
$fs = 0.4;
$fn = 100;

include <ray_trace.scad>

// camera
camera(
  aspect_ratio=16.0 / 9.0,
  image_width=600,
  samples_per_pixel=10,
  max_depth=50,
  defocus_angle=0.6,
  focus_distance=1.0,
  background=[0.7, 0.8, 1.0]
);

// ground
lambertian(t=checker(scale=0.32, even=[0.2, 0.3, 0.1], odd=[0.9, 0.9, 0.9]))
  translate([0.0, -1.0, -100.5])
    sphere(r=100);

// center
color([0.1, 0.2, 0.5])
  translate([0.0, -1.2, 0.0])
    sphere(r=0.5);

// left
dielectric(n=1.5)
  translate([1.0, -1.0, 0.0])
    sphere(r=0.5);
dielectric(n=1.0 / 1.5)
  translate([1.0, -1.0, 0.0])
    sphere(r=0.4);

// right
metal(c=[0.8, 0.6, 0.2], fuzz=0.2)
  translate([-1.0, -1.0, 0.0])
    sphere(r=0.5);
