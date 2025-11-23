use crate::{Color, Interval, Ray, RenderContext, Vector3, object::Node};

#[derive(Debug)]
pub struct CameraBuilder {
    // Vertical view angle (field of view)
    pub vertical_fov: f64,
    pub aspect_ratio: f64,
    pub image_width: u32,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            vertical_fov: 90.0,
            aspect_ratio: 16.0 / 9.0,
            image_width: 600,
        }
    }

    pub fn build(&self) -> Camera {
        let vertical_fov: f64 = 90.0;
        let center = Vector3::new(0.0, 0.0, 0.0);

        let image_height: u32 = (self.image_width as f64 / self.aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        let focal_length = 1.0;
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let samples_per_pixel = 10.0;

        Camera {
            image_width: self.image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth: 10,
            samples_per_pixel: samples_per_pixel as u32,
            pixel_samples_scale: 1.0 / samples_per_pixel,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vector3,
    pixel00_loc: Vector3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Count of random samples for each pixel
    samples_per_pixel: u32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
}

impl Camera {
    #[allow(clippy::only_used_in_recursion)]
    fn ray_color(&self, ctx: &RenderContext, ray: Ray, depth: u32, node: &dyn Node) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        // The previous intersection might be just above the surface or might be just below the surface.
        // If the ray's origin is just below the surface then it could intersect with that surface again.
        // Which means that it will find the nearest surface at t=0.00000001 or whatever floating point
        // approximation the hit function gives us. To address this raise the ray just a little bit off
        // the surface.
        if let Some(rec) = node.hit(&ray, Interval::new(0.00001, f64::INFINITY)) {
            if let Some(scatter) = rec.material.scatter(ctx, &ray, &rec) {
                let c: Color = self.ray_color(ctx, scatter.scattered, depth - 1, node);
                return scatter.attenuation * c;
            }

            return Color::BLACK;
        }

        // create a blue gradient sky
        let unit_direction = ray.direction.unit();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, ctx: &RenderContext, x: u32, y: u32, node: &dyn Node) -> Color {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _sample in 0..self.samples_per_pixel {
            let r = self.get_ray(ctx, x, y);
            pixel_color += self.ray_color(ctx, r, self.max_depth, node);
        }
        (self.pixel_samples_scale * pixel_color).linear_to_gamma()
    }

    /// Construct a camera ray originating from the origin and directed at randomly sampled
    /// point around the pixel location x,y.
    fn get_ray(&self, ctx: &RenderContext, x: u32, y: u32) -> Ray {
        let offset = Vector3::sample_square(ctx.random);
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    // let pixel_center =
    //     self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
    // let ray_direction = pixel_center - self.center;
    // let r = Ray::new(self.center, ray_direction);
    // self.ray_color(ctx, r, self.max_depth, node)
    //     .linear_to_gamma()

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }
}
