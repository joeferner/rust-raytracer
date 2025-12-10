$fa = 1;
$fs = 0.4;

include <ray_trace.scad>

// camera
camera(
    // aspect_ratio = 1.0,
    image_width = 600,
    image_height = 600,
    samples_per_pixel = 10,
    max_depth = 10,
    vertical_fov = 90.0,
    look_from = [50.0, -50.0, 70.0],
    look_at = [0.0, 0.0, 0.0],
    up = [0.0, 0.0, 1.0],
    defocus_angle = 0.0,
    focus_distance = 10.0,
    background = [178, 205, 255]
);

// axis
cube([1,1,200],center=true);
cube([1,200,1],center=true);
cube([200,1,1],center=true);

// Car body base
cube([60,20,10],center=true);

// Car body top
translate([5,0,10 - 0.001])
    cube([30,20,10],center=true);

// Front left wheel
translate([-20,-15,0])
    rotate([90,0,0])
    cylinder(h=3,r=8,center=true);

// Front right wheel
translate([-20,15,0])
    rotate([90,0,0])
    cylinder(h=3,r=8,center=true);

// Rear left wheel
translate([20,-15,0])
    rotate([90,0,0])
    cylinder(h=3,r=8,center=true);

// Rear right wheel
translate([20,15,0])
    rotate([90,0,0])
    cylinder(h=3,r=8,center=true);

// Front axle
translate([-20,0,0])
    rotate([90,0,0])
    cylinder(h=30,r=2,center=true);

// Rear axle
translate([20,0,0])
    rotate([90,0,0])
    cylinder(h=30,r=2,center=true);
