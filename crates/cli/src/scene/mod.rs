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

use rust_raytracer_core::{RenderContext, SceneData};
use rust_raytracer_openscad::openscad_file_to_scene_data;

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
    OpenScad(String),
}

pub fn get_scene(ctx: &RenderContext, scene: Scene) -> Result<SceneData, String> {
    match scene {
        Scene::ThreeSpheres => Ok(create_three_spheres_scene(ctx)),
        Scene::RandomSpheres => Ok(create_random_spheres_scene(ctx)),
        Scene::CheckeredSpheres => Ok(create_checkered_spheres_scene(ctx)),
        Scene::Earth => Ok(create_earth_scene(ctx)),
        Scene::PerlinSpheres => Ok(create_perlin_spheres_scene(ctx)),
        Scene::Quads => Ok(create_quads_scene(ctx)),
        Scene::SimpleLight => Ok(create_simple_light_scene(ctx)),
        Scene::CornellBox => Ok(create_cornell_box_scene(ctx)),
        Scene::CornellBoxSmoke => Ok(create_cornell_box_smoke_scene(ctx)),
        Scene::Final => Ok(create_final_scene(ctx)),
        Scene::OpenScad(filename) => {
            openscad_file_to_scene_data(&filename).map_err(|err| format!("{err:?}"))
        }
    }
}
