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
  vertical_fov=20.0,
  look_from=[-13.0, 3.0, 2.0],
  defocus_angle=0.6,
  focus_distance=10.0,
  background=[0.7, 0.8, 1.0]
);

// ground
lambertian(c=[0.5, 0.5, 0.5])
  translate([0.0, 0.0, -1000.0])
    sphere(r=1000);

// distance function
function distance(pt1, pt2) = sqrt(pow(pt2[0]-pt1[0], 2) + pow(pt2[1]-pt1[1], 2) + pow(pt2[2]-pt1[2], 2));

// random spheres
for(a = [-11 : 11]) {
    for(b = [-11 : 11]) {
        choose_mat = rands(0,1,1)[0];
        center = [
              a + 0.9 * rands(0,1,1)[0],
              0.2,
              b + 0.9 * rands(0,1,1)[0],
        ];

        if (distance(center, [4.0, 0.2, 0.0]) > 0.9) {
          // if choose_mat < 0.8 {
          //     // diffuse
          //     let albedo = Color::random(&*ctx.random) * Color::random(&*ctx.random);
          //     let sphere_material = Arc::new(Lambertian::new_from_color(albedo));
          //     let mut sphere = Sphere::new(center, 0.2, sphere_material);
          //     sphere.set_direction(Vector3::new(
          //         0.0,
          //         ctx.random.rand_interval(0.0, 0.5),
          //         0.0,
          //     ));
          //     world.push(Arc::new(sphere));
          // } else if choose_mat < 0.95 {
          //     // metal
          //     let albedo = Color::random_interval(&*ctx.random, 0.5, 1.0);
          //     let fuzz = ctx.random.rand_interval(0.0, 0.5);
          //     let sphere_material = Arc::new(Metal::new(albedo, fuzz));
          //     world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
          // } else {
          //     // glass
          //     let sphere_material = Arc::new(Dielectric::new(1.5));
          //     world.push(Arc::new(Sphere::new(center, 0.2, sphere_material)));
          // }
        }
    }
}

// large spheres
dielectric(1.5)
  translate([0.0, 0.0, 1.0])
    sphere(r=1.0);
color([0.4, 0.2, 0.1])
  translate([4.0, 0.0, 1.0])
    sphere(r=1.0);
metal([0.7, 0.6, 0.5], 0.0)
  translate([-4.0, 0.0, 1.0])
    sphere(r=1.0);
