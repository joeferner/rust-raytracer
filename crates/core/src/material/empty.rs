use serde::{Deserialize, Serialize};

use crate::material::Material;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmptyMaterial {}

impl EmptyMaterial {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for EmptyMaterial {
    fn default() -> Self {
        EmptyMaterial::new()
    }
}

#[typetag::serde]
impl Material for EmptyMaterial {
    fn scatter(
        &self,
        _ctx: &crate::RenderContext,
        _r_in: &crate::Ray,
        _hit: &crate::object::HitRecord,
    ) -> Option<super::ScatterResult> {
        None
    }
}
