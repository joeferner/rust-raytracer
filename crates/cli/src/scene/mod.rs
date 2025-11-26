#![allow(clippy::vec_init_then_push)]
pub mod checkered_spheres;
pub mod random_spheres;
pub mod three_spheres;

use std::sync::Arc;

use rust_raytracer_core::{Camera, RenderContext, object::Node};

use crate::scene::{
    checkered_spheres::get_scene_checkered_spheres, random_spheres::get_scene_random_spheres,
    three_spheres::get_scene_three_spheres,
};

pub enum Scene {
    ThreeSpheres,
    RandomSpheres,
    CheckeredSpheres,
}

pub fn get_scene(ctx: &RenderContext, scene: Scene) -> (Arc<Camera>, Arc<dyn Node>) {
    match scene {
        Scene::ThreeSpheres => get_scene_three_spheres(ctx),
        Scene::RandomSpheres => get_scene_random_spheres(ctx),
        Scene::CheckeredSpheres => get_scene_checkered_spheres(ctx),
    }
}
