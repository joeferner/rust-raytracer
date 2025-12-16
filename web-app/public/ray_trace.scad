module camera(
  aspect_ratio = 1.0,
  image_width = 100,
  samples_per_pixel = 10,
  max_depth = 10,
  vertical_fov = 90.0,
  defocus_angle = 0.0,
  focus_distance = 10.0,
  background = [0, 0, 0],
  look_from = [0, 0, 0],
  look_at = [0, 0, 0],
  up = [0, 1, 0],
  image_height
){}

module lambertian(c, t) {
  children();
}

module dielectric(n) {
  children();
}

module metal(c, fuzz) {
  children();
}

function checker(scale = 1, even = [0, 0, 0], odd = [1, 1, 1]) = even;
