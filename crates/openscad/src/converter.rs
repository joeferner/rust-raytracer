use std::{rc::Rc, sync::Arc};

use rust_raytracer_core::{
    Camera, CameraBuilder, Color, Node, SceneData, Vector3,
    material::Lambertian,
    object::{BoundingVolumeHierarchy, Box},
};

use crate::interpreter::{
    Module, ModuleArgument, ModuleArgumentValue, ModuleInstance, ModuleInstanceTree,
};

struct Converter {
    camera: Option<Arc<Camera>>,
    world: Vec<Arc<dyn Node>>,
    lights: Vec<Arc<dyn Node>>,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            camera: None,
            world: vec![],
            lights: vec![],
        }
    }

    fn convert(mut self, modules: Vec<Rc<ModuleInstanceTree>>) -> SceneData {
        for module in modules {
            let node = self.process_module(module);
            self.world.push(node);
        }

        let camera = if let Some(camera) = self.camera {
            camera
        } else {
            let mut camera_builder = CameraBuilder::new();
            camera_builder.aspect_ratio = 1.0;
            camera_builder.image_width = 600;
            camera_builder.samples_per_pixel = 10;
            camera_builder.max_depth = 50;
            camera_builder.defocus_angle = 0.0;
            camera_builder.background = Color::new(0.7, 0.8, 1.0);
            camera_builder.look_at = Vector3::new(0.0, 0.0, 0.0);
            camera_builder.look_from = Vector3::new(20.0, 20.0, 20.0);
            camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
            Arc::new(camera_builder.build())
        };

        SceneData {
            camera,
            world: Arc::new(BoundingVolumeHierarchy::new(&self.world)),
            lights: if self.lights.is_empty() {
                None
            } else {
                Some(Arc::new(BoundingVolumeHierarchy::new(&self.lights)))
            },
        }
    }

    fn process_module(&mut self, module: Rc<ModuleInstanceTree>) -> Arc<dyn Node> {
        let mut child_nodes: Vec<Arc<dyn Node>> = vec![];

        for child_module in &module.children {
            let child_node = self.process_module(child_module.clone());
            child_nodes.push(child_node);
        }

        self.process_module_instance(&module.instance, child_nodes)
    }

    fn process_module_instance(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Arc<dyn Node> {
        match instance.module {
            Module::Cube => {
                if !child_nodes.is_empty() {
                    todo!();
                }
                self.create_cube(&instance.arguments)
            }
        }
    }

    fn create_cube(&self, arguments: &[ModuleArgument]) -> Arc<dyn Node> {
        let mut size_x = 1.0;
        let mut size_y = 1.0;
        let mut size_z = 1.0;
        let center = false;

        if arguments.len() != 1 {
            todo!();
        }
        match &arguments[0] {
            ModuleArgument::Positional(value) => match &value {
                ModuleArgumentValue::Number(value) => {
                    size_x = *value;
                    size_y = *value;
                    size_z = *value;
                }
            },
        }

        let (a, b) = if center {
            todo!()
        } else {
            (
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(size_x, size_y, size_z),
            )
        };

        Arc::new(Box::new(
            a,
            b,
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26))),
        ))
    }
}

pub fn openscad_convert(modules: Vec<Rc<ModuleInstanceTree>>) -> SceneData {
    let converter = Converter::new();
    converter.convert(modules)
}
