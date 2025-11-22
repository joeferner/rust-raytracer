use crate::{
    Color, Ray, RenderContext, Vector3,
    material::{Material, ScatterResult},
    object::HitRecord,
};

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ctx: &RenderContext, r_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = r_in.direction.reflect(hit.normal);
        let reflected = reflected.unit() + (self.fuzz * Vector3::random_unit(ctx));
        let scattered = Ray::new(hit.pt, reflected);
        if scattered.direction.dot(&hit.normal) > 0.0 {
            Some(ScatterResult {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}
