use std::sync::Arc;

use rust_raytracer_core::{
    CameraBuilder, Color, Node, RenderContext, Vector3,
    material::{Dielectric, DiffuseLight, EmptyMaterial, Lambertian},
    object::{BoundingVolumeHierarchy, BoxPrimitive, Group, Quad, Rotate, Sphere, Translate},
};

use crate::scene::SceneData;

pub fn create_cornell_box_scene(_ctx: &RenderContext) -> SceneData {
    let red_material = Arc::new(Lambertian::new_from_color(Color::new(0.65, 0.05, 0.05)));
    let white_material = Arc::new(Lambertian::new_from_color(Color::new(0.73, 0.73, 0.73)));
    let green_material = Arc::new(Lambertian::new_from_color(Color::new(0.12, 0.45, 0.15)));
    let light_material = Arc::new(DiffuseLight::new_from_color(Color::new(15.0, 15.0, 15.0)));

    // World
    let mut world: Vec<Arc<dyn Node>> = vec![];

    world.push(Arc::new(Quad::new(
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        green_material,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        red_material,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        light_material,
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 555.0),
        white_material.clone(),
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(555.0, 555.0, 555.0),
        Vector3::new(-555.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -555.0),
        white_material.clone(),
    )));
    world.push(Arc::new(Quad::new(
        Vector3::new(0.0, 0.0, 555.0),
        Vector3::new(555.0, 0.0, 0.0),
        Vector3::new(0.0, 555.0, 0.0),
        white_material.clone(),
    )));

    // box1
    let box1 = Arc::new(BoxPrimitive::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(165.0, 330.0, 165.0),
        white_material.clone(),
    ));
    let box1 = Arc::new(Rotate::rotate_y(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vector3::new(265.0, 0.0, 295.0)));
    world.push(box1);

    // box2
    // let box2 = Arc::new(Box::new(
    //     Vector3::new(0.0, 0.0, 0.0),
    //     Vector3::new(165.0, 165.0, 165.0),
    //     white_material,
    // ));
    // let box2 = Arc::new(RotateY::new(box2, -18.0));
    // let box2 = Arc::new(Translate::new(box2, Vector3::new(130.0, 0.0, 65.0)));
    // world.push(box2);
    let glass = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Vector3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    let world = Arc::new(BoundingVolumeHierarchy::new(&world));

    // Lights
    let light1 = Arc::new(Quad::new(
        Vector3::new(343.0, 554.0, 332.0),
        Vector3::new(-130.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, -105.0),
        Arc::new(EmptyMaterial::new()),
    ));
    let light2 = Arc::new(Sphere::new(
        Vector3::new(190.0, 90.0, 190.0),
        90.0,
        Arc::new(EmptyMaterial::new()),
    ));
    let lights: Vec<Arc<dyn Node>> = vec![light1, light2];
    let lights = Arc::new(Group::from_list(&lights));

    // Camera
    let mut camera_builder = CameraBuilder::new();
    camera_builder.aspect_ratio = 1.0;
    camera_builder.image_width = 600;
    camera_builder.samples_per_pixel = 100;
    camera_builder.max_depth = 50;
    camera_builder.vertical_fov = 40.0;
    camera_builder.look_from = Vector3::new(278.0, 278.0, -800.0);
    camera_builder.look_at = Vector3::new(278.0, 278.0, 0.0);
    camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
    camera_builder.background = Color::BLACK;
    camera_builder.defocus_angle = 0.0;
    let camera = Arc::new(camera_builder.build());

    SceneData {
        camera,
        world,
        lights: Some(lights),
    }
}
