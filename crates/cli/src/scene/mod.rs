#![allow(clippy::vec_init_then_push)]
pub mod checkered_spheres;
pub mod cornell_box;
pub mod cornell_box_smoke;
pub mod earth;
pub mod final_scene;
pub mod perlin_spheres;
pub mod quads;
pub mod random_spheres;
pub mod simple_light;
pub mod three_spheres;

use std::sync::Arc;

use rust_raytracer_core::{Camera, RenderContext, object::Node};

use crate::scene::{
    checkered_spheres::create_checkered_spheres_scene, cornell_box::create_cornell_box_scene,
    cornell_box_smoke::create_cornell_box_smoke_scene, earth::create_earth_scene,
    final_scene::create_final_scene, perlin_spheres::create_perlin_spheres_scene,
    quads::create_quads_scene, random_spheres::create_random_spheres_scene,
    simple_light::create_simple_light_scene, three_spheres::create_three_spheres_scene,
};

pub enum Scene {
    ThreeSpheres,
    RandomSpheres,
    CheckeredSpheres,
    Earth,
    PerlinSpheres,
    Quads,
    SimpleLight,
    CornellBox,
    CornellBoxSmoke,
    Final,
}

pub struct SceneResult {
    pub camera: Arc<Camera>,
    pub world: Arc<dyn Node>,
    pub lights: Option<Arc<dyn Node>>,
}

pub fn get_scene(ctx: &RenderContext, scene: Scene) -> SceneResult {
    match scene {
        Scene::ThreeSpheres => create_three_spheres_scene(ctx),
        Scene::RandomSpheres => create_random_spheres_scene(ctx),
        Scene::CheckeredSpheres => create_checkered_spheres_scene(ctx),
        Scene::Earth => create_earth_scene(ctx),
        Scene::PerlinSpheres => create_perlin_spheres_scene(ctx),
        Scene::Quads => create_quads_scene(ctx),
        Scene::SimpleLight => create_simple_light_scene(ctx),
        Scene::CornellBox => create_cornell_box_scene(ctx),
        Scene::CornellBoxSmoke => create_cornell_box_smoke_scene(ctx),
        Scene::Final => create_final_scene(ctx),
    }
}
