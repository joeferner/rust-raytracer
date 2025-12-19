use std::{f64, sync::Arc};

use crate::{
    Color, HittablePdf, Interval, Random, Ray, RenderContext, Vector3, material::PdfOrRay,
    object::Node, probability_density_function::MixturePdf,
};

/// Builder for configuring and constructing a [`Camera`].
///
/// The `CameraBuilder` uses the builder pattern to configure camera parameters
/// before creating the camera. It provides sensible defaults for all parameters,
/// which can be overridden as needed.
///
/// # Examples
///
/// ```
/// use rust_raytracer_core::{CameraBuilder, Vector3, Color};
///
/// let mut camera_builder = CameraBuilder::new();
/// camera_builder.aspect_ratio = 16.0 / 9.0;
/// camera_builder.image_width = 1920;
/// camera_builder.samples_per_pixel = 100;
/// camera_builder.max_depth = 50;
/// camera_builder.vertical_fov = 20.0;
/// camera_builder.look_from = Vector3::new(13.0, 2.0, 3.0);
/// camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
/// camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
/// camera_builder.defocus_angle = 0.6;
/// camera_builder.focus_distance = 10.0;
/// camera_builder.background = Color::new(0.7, 0.8, 1.0);
/// let camera = camera_builder.build();
/// ```
#[derive(Debug)]
pub struct CameraBuilder {
    /// Vertical view angle (field of view) in degrees.
    ///
    /// Controls the camera's zoom level. Smaller values create a "zoomed in" effect,
    /// while larger values create a wide-angle view.
    pub vertical_fov: f64,

    /// Ratio of image width over height.
    ///
    /// Common aspect ratios include 16:9 (1.777...), 4:3 (1.333...), and 1:1.
    pub aspect_ratio: f64,

    /// Rendered image width in pixel count.
    ///
    /// The image height is automatically calculated from the aspect ratio.
    pub image_width: u32,

    /// Point camera is looking from (camera position).
    pub look_from: Vector3,

    /// Point camera is looking at (target position).
    pub look_at: Vector3,

    /// Camera-relative "up" direction.
    ///
    /// Defines the camera's roll orientation. Typically (0, 1, 0) for an upright camera.
    pub up: Vector3,

    /// Variation angle of rays through each pixel in degrees.
    ///
    /// Controls depth of field blur. A value of 0 means everything is in focus.
    /// Larger values create more pronounced depth of field effects.
    pub defocus_angle: f64,

    /// Distance from camera look_from point to plane of perfect focus.
    ///
    /// Objects at this distance will be perfectly sharp, while objects closer
    /// or farther will be progressively blurred based on the defocus_angle.
    pub focus_distance: f64,

    /// Count of random samples for each pixel.
    ///
    /// Higher values produce smoother, less noisy images but take longer to render.
    pub samples_per_pixel: u32,

    /// Maximum number of ray bounces into scene.
    ///
    /// Limits recursion depth to prevent infinite loops and control render time.
    /// Higher values allow more light bounces but increase computation.
    pub max_depth: u32,

    /// Scene background color.
    ///
    /// Color returned when a ray doesn't hit any objects in the scene.
    pub background: Color,
}

impl CameraBuilder {
    /// Creates a new `CameraBuilder` with default values.
    ///
    /// # Default Values
    /// - aspect_ratio: 1.0 (square)
    /// - image_width: 100 pixels
    /// - samples_per_pixel: 10
    /// - max_depth: 10 bounces
    /// - background: black (0, 0, 0)
    /// - vertical_fov: 90 degrees
    /// - look_from: (0, 0, 0)
    /// - look_at: (0, 0, -1)
    /// - up: (0, 1, 0)
    /// - defocus_angle: 0 (no depth of field)
    /// - focus_distance: 10
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

    /// Constructs a [`Camera`] from the current builder configuration.
    ///
    /// # Returns
    /// A fully configured [`Camera`] ready for rendering.
    pub fn build(&self) -> Camera {
        let image_height: u32 = (self.image_width as f64 / self.aspect_ratio) as u32;
        let image_height: u32 = if image_height < 1 { 1 } else { image_height };

        // Calculate stratified sampling parameters
        let sqrt_spp = (self.samples_per_pixel as f64).sqrt() as u32;
        let pixel_samples_scale = 1.0 / (sqrt_spp * sqrt_spp) as f64;
        let reciprocal_sqrt_spp = 1.0 / sqrt_spp as f64;

        let center = self.look_from;

        // Calculate viewport dimensions based on field of view
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
            sqrt_spp,
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

/// A camera that renders 3D scenes.
///
/// The `Camera` struct represents a configured camera ready to render images.
/// It supports features like:
/// - Configurable field of view
/// - Depth of field effects (defocus blur)
/// - Stratified sampling for anti-aliasing
/// - Path tracing with importance sampling
///
/// Use [`CameraBuilder`] to construct a `Camera` instance.
#[derive(Debug)]
pub struct Camera {
    /// Rendered image width in pixels
    image_width: u32,
    /// Rendered image height in pixels
    image_height: u32,
    /// Camera center position in world space
    center: Vector3,
    /// Location of pixel (0, 0) in world space
    pixel00_loc: Vector3,
    /// Offset vector to pixel to the right
    pixel_delta_u: Vector3,
    /// Offset vector to pixel below
    pixel_delta_v: Vector3,
    /// Maximum number of ray bounces into scene
    max_depth: u32,
    /// Color scale factor for a sum of pixel samples (1 / samples_per_pixel)
    pixel_samples_scale: f64,
    /// Variation angle of rays through each pixel in degrees
    defocus_angle: f64,
    /// Defocus disk horizontal radius vector
    defocus_disk_u: Vector3,
    /// Defocus disk vertical radius vector
    defocus_disk_v: Vector3,
    /// Scene background color for rays that miss all objects
    background: Color,
    /// Square root of number of samples per pixel
    sqrt_spp: u32,
    /// Reciprocal of sqrt_spp (1 / sqrt_spp)
    reciprocal_sqrt_spp: f64,
}

impl Camera {
    /// Traces a ray through the scene and calculates its color.
    ///
    /// This method recursively traces rays through the scene, accumulating color
    /// from emissive materials and scattered light. It uses importance sampling
    /// with a mixture of material and light PDFs for efficient rendering.
    ///
    /// # Parameters
    /// - `ctx`: Rendering context containing random number generator
    /// - `ray`: The ray to trace
    /// - `depth`: Remaining recursion depth
    /// - `world`: The scene geometry to test for intersections
    /// - `lights`: Light sources for importance sampling
    ///
    /// # Returns
    /// The color seen along the ray direction.
    #[allow(clippy::only_used_in_recursion)]
    fn ray_color(
        &self,
        ctx: &RenderContext,
        ray: Ray,
        depth: u32,
        world: &dyn Node,
        lights: Option<Arc<dyn Node>>,
    ) -> Color {
        // Recursion limit reached
        if depth == 0 {
            return Color::BLACK;
        }

        // If the ray hits nothing, return the background color.
        let Some(hit) = world.hit(ctx, &ray, Interval::new(0.001, f64::INFINITY)) else {
            return self.background;
        };

        let color_from_emission = hit.material.emitted(&ray, &hit, hit.u, hit.v, hit.pt);

        match hit.material.scatter(ctx, &ray, &hit) {
            None => color_from_emission,
            Some(scatter_results) => match scatter_results.pdf_or_ray {
                // Specular reflection (delta distribution)
                PdfOrRay::Ray(ray) => {
                    scatter_results.attenuation * self.ray_color(ctx, ray, depth - 1, world, lights)
                }
                // Diffuse/glossy reflection (use importance sampling)
                PdfOrRay::Pdf(material_pdf) => {
                    let pdf = match &lights {
                        Some(lights) => {
                            let light_pdf = Arc::new(HittablePdf::new(lights.clone(), hit.pt));
                            Arc::new(MixturePdf::new(light_pdf, material_pdf))
                        }
                        None => material_pdf,
                    };

                    let scattered = Ray::new_with_time(hit.pt, pdf.generate(ctx), ray.time);
                    let pdf_value = pdf.value(ctx, &scattered.direction);

                    // Guard against small or invalid PDF values which can cause over exposure
                    if pdf_value < 0.05 {
                        return color_from_emission;
                    }

                    let scattering_pdf = hit.material.scattering_pdf(ctx, &ray, &hit, &scattered);

                    let sample_color = self.ray_color(ctx, scattered, depth - 1, world, lights);
                    let color_from_scatter =
                        (scatter_results.attenuation * scattering_pdf * sample_color) / pdf_value;

                    let color = color_from_emission + color_from_scatter;

                    // Clamp to prevent fireflies
                    color.clamp(0.0, 10.0)
                }
            },
        }
    }

    /// Renders a single pixel at the given coordinates.
    ///
    /// This method performs stratified sampling over the pixel area, tracing
    /// multiple rays per pixel and averaging the results for anti-aliasing.
    ///
    /// # Parameters
    /// - `ctx`: Rendering context containing random number generator
    /// - `x`: Pixel x-coordinate (0 to image_width - 1)
    /// - `y`: Pixel y-coordinate (0 to image_height - 1)
    /// - `world`: The scene geometry to render
    /// - `lights`: Light sources for importance sampling
    ///
    /// # Returns
    /// The final gamma-corrected color for the pixel.
    pub fn render(
        &self,
        ctx: &RenderContext,
        x: u32,
        y: u32,
        world: &dyn Node,
        lights: Option<Arc<dyn Node>>,
    ) -> Color {
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        // Stratified sampling: divide pixel into sqrt_spp x sqrt_spp grid
        for s_y in 0..self.sqrt_spp {
            for s_x in 0..self.sqrt_spp {
                let r = self.get_ray(ctx, x, y, s_x, s_y);
                let sample = self.ray_color(ctx, r, self.max_depth, world, lights.clone());
                pixel_color += sample;
            }
        }

        let pixel_color = self.pixel_samples_scale * pixel_color.nan_to_zero();
        pixel_color.linear_to_gamma()
    }

    /// Constructs a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel location (x, y).
    ///
    /// # Parameters
    /// - `ctx`: Rendering context containing random number generator
    /// - `x`: Pixel x-coordinate
    /// - `y`: Pixel y-coordinate
    /// - `s_x`: Stratification grid x-index
    /// - `s_y`: Stratification grid y-index
    ///
    /// # Returns
    /// A ray from the camera through the specified pixel sample.
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
    ///
    /// This implements stratified sampling to reduce variance compared to pure
    /// random sampling.
    ///
    /// # Parameters
    /// - `random`: Random number generator
    /// - `s_x`: Stratification grid x-index (0 to sqrt_spp - 1)
    /// - `s_y`: Stratification grid y-index (0 to sqrt_spp - 1)
    ///
    /// # Returns
    /// A random offset within the specified sub-pixel region.
    fn sample_square_stratified(&self, random: &dyn Random, s_x: u32, s_y: u32) -> Vector3 {
        let px = ((s_x as f64 + random.rand()) * self.reciprocal_sqrt_spp) - 0.5;
        let py = ((s_y as f64 + random.rand()) * self.reciprocal_sqrt_spp) - 0.5;

        Vector3::new(px, py, 0.0)
    }

    /// Returns the rendered image width in pixels.
    pub fn image_width(&self) -> u32 {
        self.image_width
    }

    /// Returns the rendered image height in pixels.
    pub fn image_height(&self) -> u32 {
        self.image_height
    }

    /// Returns a random point in the camera defocus disk.
    ///
    /// This is used to create depth of field effects by varying the ray origin
    /// across a disk perpendicular to the view direction.
    ///
    /// # Parameters
    /// - `random`: Random number generator
    ///
    /// # Returns
    /// A random point on the defocus disk in world space.
    fn defocus_disk_sample(&self, random: &dyn Random) -> Vector3 {
        let pt = Vector3::random_in_unit_disk(random);
        self.center + (pt.x * self.defocus_disk_u) + (pt.y * self.defocus_disk_v)
    }
}
