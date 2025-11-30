use std::{f64, sync::Arc};

use crate::{
    Color, HittablePdf, Interval, ProbabilityDensityFunction, Random, Ray, RenderContext, Vector3,
    material::PdfOrRay, object::Node, probability_density_function::MixturePdf,
};

#[derive(Debug)]
pub struct CameraBuilder {
    /// Vertical view angle (field of view) (degrees)
    pub vertical_fov: f64,
    /// Ratio of image width over height
    pub aspect_ratio: f64,
    /// Rendered image width in pixel count
    pub image_width: u32,
    /// Point camera is looking from
    pub look_from: Vector3,
    /// Point camera is looking at
    pub look_at: Vector3,
    /// Camera-relative "up" direction
    pub up: Vector3,
    /// Variation angle of rays through each pixel (degrees)
    pub defocus_angle: f64,
    // Distance from camera look_from point to plane of perfect focus
    pub focus_distance: f64,
    /// Count of random samples for each pixel
    pub samples_per_pixel: u32,
    /// Maximum number of ray bounces into scene
    pub max_depth: u32,
    /// Scene background color
    pub background: Color,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            background: Color::new(0.0, 0.0, 0.0),
            vertical_fov: 90.0,
            look_from: Vector3::new(0.0, 0.0, 0.0),
            look_at: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
        }
    }

    pub fn build(&self) -> Camera {
        let image_height: u32 = (self.image_width as f64 / self.aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        let sqrt_spp = (self.samples_per_pixel as f64).sqrt();
        let pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp);
        let reciprocal_sqrt_spp = 1.0 / sqrt_spp;

        let center = self.look_from;

        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (self.look_from - self.look_at).unit();
        let u = self.up.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -v; // Vector down viewport vertical edge

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - (self.focus_distance * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_distance * ((self.defocus_angle / 2.0).to_radians()).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width: self.image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            max_depth: self.max_depth,
            defocus_angle: self.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            background: self.background,
            sqrt_spp: sqrt_spp as u32,
            reciprocal_sqrt_spp,
            pixel_samples_scale,
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
    /// Rendered image height
    image_height: u32,
    /// Camera center
    center: Vector3,
    /// Location of pixel 0, 0
    pixel00_loc: Vector3,
    /// Offset to pixel to the right
    pixel_delta_u: Vector3,
    /// Offset to pixel below
    pixel_delta_v: Vector3,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Color scale factor for a sum of pixel samples
    pixel_samples_scale: f64,
    /// Variation angle of rays through each pixel (degrees)
    defocus_angle: f64,
    /// Defocus disk horizontal radius
    defocus_disk_u: Vector3,
    /// Defocus disk vertical radius
    defocus_disk_v: Vector3,
    /// Scene background color
    background: Color,
    /// Square root of number of samples per pixel
    sqrt_spp: u32,
    /// 1 / sqrt_spp
    reciprocal_sqrt_spp: f64,
}

impl Camera {
    #[allow(clippy::only_used_in_recursion)]
    fn ray_color(
        &self,
        ctx: &RenderContext,
        ray: Ray,
        depth: u32,
        world: &dyn Node,
        lights: Arc<dyn Node>,
    ) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        // If the ray hits nothing, return the background color.
        let Some(hit) = world.hit(ctx, &ray, Interval::new(0.001, f64::INFINITY)) else {
            return self.background;
        };

        let color_from_emission = hit.material.emitted(&ray, &hit, hit.u, hit.v, hit.pt);

        let (attenuation, pdf_or_ray) = {
            let Some(scatter_result) = hit.material.scatter(ctx, &ray, &hit) else {
                return color_from_emission;
            };
            (scatter_result.attenuation, scatter_result.pdf_or_ray)
        };

        match pdf_or_ray {
            PdfOrRay::Ray(ray) => attenuation * self.ray_color(ctx, ray, depth - 1, world, lights),
            PdfOrRay::Pdf(pdf) => {
                let light_pdf = Arc::new(HittablePdf::new(lights.clone(), hit.pt));
                let p = MixturePdf::new(light_pdf, pdf);

                let scattered = Ray::new_with_time(hit.pt, p.generate(ctx), ray.time);
                let pdf_value = p.value(ctx, &scattered.direction);

                let scattering_pdf = hit.material.scattering_pdf(ctx, &ray, &hit, &scattered);

                let sample_color = self.ray_color(ctx, scattered, depth - 1, world, lights);
                let color_from_scatter = (attenuation * scattering_pdf * sample_color) / pdf_value;

                color_from_emission + color_from_scatter
            }
        }
    }

    pub fn render(
        &self,
        ctx: &RenderContext,
        x: u32,
        y: u32,
        world: &dyn Node,
        lights: Arc<dyn Node>,
    ) -> Color {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for s_y in 0..self.sqrt_spp {
            for s_x in 0..self.sqrt_spp {
                let r = self.get_ray(ctx, x, y, s_x, s_y);
                pixel_color += self.ray_color(ctx, r, self.max_depth, world, lights.clone());
            }
        }

        (self.pixel_samples_scale * pixel_color.nan_to_zero()).linear_to_gamma()
    }

    /// Construct a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, ctx: &RenderContext, x: u32, y: u32, s_x: u32, s_y: u32) -> Ray {
        let offset = self.sample_square_stratified(&*ctx.random, s_x, s_y);
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x) * self.pixel_delta_u)
            + ((y as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(&*ctx.random)
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = ctx.random.rand();

        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    /// Returns the vector to a random point in the square sub-pixel specified by grid
    /// indices s_x and s_y, for an idealized unit square pixel [-.5,-.5] to [+.5,+.5].
    fn sample_square_stratified(&self, random: &dyn Random, s_x: u32, s_y: u32) -> Vector3 {
        let px = ((s_x as f64 + random.rand()) * self.reciprocal_sqrt_spp) - 0.5;
        let py = ((s_y as f64 + random.rand()) * self.reciprocal_sqrt_spp) - 0.5;

        Vector3::new(px, py, 0.0)
    }

    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    /// Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self, random: &dyn Random) -> Vector3 {
        let pt = Vector3::random_in_unit_disk(random);
        self.center + (pt.x * self.defocus_disk_u) + (pt.y * self.defocus_disk_v)
    }
}
