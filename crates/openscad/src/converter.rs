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
            if let Some(node) = self.process_module(module) {
                self.world.push(node);
            }
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
            camera_builder.look_from = Vector3::new(-50.0, 70.0, -50.0);
            camera_builder.up = Vector3::new(0.0, 1.0, 0.0);
            Arc::new(camera_builder.build())
        };

        // TODO check for errors

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

    fn process_module(&mut self, module: Rc<ModuleInstanceTree>) -> Option<Arc<dyn Node>> {
        let mut child_nodes: Vec<Arc<dyn Node>> = vec![];

        for child_module in &module.children {
            if let Some(child_node) = self.process_module(child_module.clone()) {
                child_nodes.push(child_node);
            } else {
                return None;
            }
        }

        self.process_module_instance(&module.instance, child_nodes)
    }

    fn process_module_instance(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        match instance.module {
            Module::Cube => {
                if !child_nodes.is_empty() {
                    todo!();
                }
                self.create_cube(&instance.arguments)
            }
        }
    }

    fn create_cube(&self, arguments: &[ModuleArgument]) -> Option<Arc<dyn Node>> {
        let mut size = Vector3::new(0.0, 0.0, 0.0);
        let mut center = false;

        for (pos, argument) in arguments.iter().enumerate() {
            match &argument {
                ModuleArgument::Positional(value) => {
                    if pos == 0 {
                        if let Some(v) = self.module_argument_value_to_vector3(value) {
                            size = v;
                        } else {
                            return None;
                        }
                    }
                }
                ModuleArgument::NamedArgument { name, value } => {
                    if name == "size" {
                        todo!();
                    } else if name == "center" {
                        if let Some(value) = self.module_argument_value_to_boolean(value) {
                            center = value;
                        } else {
                            return None;
                        }
                    } else {
                        todo!();
                    }
                }
            }
        }

        let mut a = Vector3::new(0.0, 0.0, 0.0);
        let mut b = size;
        if center {
            a = a - (size / 2.0);
            b = b - (size / 2.0);
        }

        Some(Arc::new(Box::new(
            a,
            b,
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26))),
        )))
    }

    fn vector_expr_to_vector3(&self, items: &[ModuleArgumentValue]) -> Option<Vector3> {
        if items.len() != 3 {
            todo!();
        }

        let x = if let ModuleArgumentValue::Number(x) = items[0] {
            x
        } else {
            todo!();
        };

        let y = if let ModuleArgumentValue::Number(y) = items[1] {
            y
        } else {
            todo!();
        };

        let z = if let ModuleArgumentValue::Number(z) = items[2] {
            z
        } else {
            todo!();
        };

        // OpenSCAD x,y,z is different than ours so flip z and y
        Some(Vector3::new(-x, z, y))
    }

    fn module_argument_value_to_vector3(&self, value: &ModuleArgumentValue) -> Option<Vector3> {
        match &value {
            ModuleArgumentValue::Number(value) => Some(Vector3::new(-*value, *value, *value)),
            ModuleArgumentValue::Vector { items } => self.vector_expr_to_vector3(items),
            _ => todo!(),
        }
    }

    fn module_argument_value_to_boolean(&self, value: &ModuleArgumentValue) -> Option<bool> {
        match value {
            ModuleArgumentValue::True => Some(true),
            ModuleArgumentValue::False => Some(false),
            _ => todo!(),
        }
    }
}

pub fn openscad_convert(modules: Vec<Rc<ModuleInstanceTree>>) -> SceneData {
    let converter = Converter::new();
    converter.convert(modules)
}
