use std::{collections::HashMap, rc::Rc, sync::Arc};

use rust_raytracer_core::{
    Camera, CameraBuilder, Color, Node, SceneData, Vector3,
    material::Lambertian,
    object::{BoundingVolumeHierarchy, BoxPrimitive, Frustum, Group, Translate},
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

        for child_module in module.children.borrow().iter() {
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
            Module::Cube => self.create_cube(instance, child_nodes),
            Module::Cylinder => self.create_cylinder(instance, child_nodes),
            Module::Translate => self.create_translate(instance, child_nodes),
        }
    }

    fn convert_args<'a>(
        &self,
        arg_names: &[&str],
        arguments: &'a [ModuleArgument],
    ) -> HashMap<String, &'a ModuleArgumentValue> {
        let mut results: HashMap<String, &'a ModuleArgumentValue> = HashMap::new();

        let mut found_named_arg = false;
        for (pos, arg) in arguments.iter().enumerate() {
            match arg {
                ModuleArgument::Positional(value) => {
                    if found_named_arg {
                        todo!("add error, no positional args after named arg");
                    }
                    if let Some(arg_name) = arg_names.get(pos) {
                        results.insert(arg_name.to_string(), value);
                    } else {
                        todo!("arg past end of list");
                    }
                }
                ModuleArgument::NamedArgument { name, value } => {
                    found_named_arg = true;
                    if arg_names.contains(&name.as_str()) {
                        results.insert(name.to_string(), value);
                    } else {
                        todo!("unknown arg name");
                    }
                }
            }
        }

        results
    }

    fn create_cube(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!();
        }

        let mut size = Vector3::new(0.0, 0.0, 0.0);
        let mut center = false;

        let arguments = self.convert_args(&["size", "center"], &instance.arguments);

        if let Some(arg) = arguments.get("size") {
            size = self.module_argument_value_to_vector3(arg)?;
        }

        if let Some(arg) = arguments.get("center") {
            center = self.module_argument_value_to_boolean(arg)?;
        }

        let mut a = Vector3::new(0.0, 0.0, 0.0);
        let mut b = size;
        if center {
            a = a - (size / 2.0);
            b = b - (size / 2.0);
        }

        Some(Arc::new(BoxPrimitive::new(
            a,
            b,
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26))),
        )))
    }

    fn create_cylinder(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!();
        }

        let mut height = 1.0;
        let mut radius1 = 1.0;
        let mut radius2 = 1.0;
        let mut center = false;

        let arguments = self.convert_args(
            &["h", "r1", "r2", "center", "r", "d", "d1", "d2"],
            &instance.arguments,
        );

        if let Some(arg) = arguments.get("h") {
            height = self.module_argument_value_to_number(arg)?;
        }

        if let Some(arg) = arguments.get("r1") {
            radius1 = self.module_argument_value_to_number(arg)?;
        }

        if let Some(arg) = arguments.get("r2") {
            radius2 = self.module_argument_value_to_number(arg)?;
        }

        if let Some(arg) = arguments.get("r") {
            let r = self.module_argument_value_to_number(arg)?;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("d1") {
            radius1 = self.module_argument_value_to_number(arg)? / 2.0;
        }

        if let Some(arg) = arguments.get("d2") {
            radius2 = self.module_argument_value_to_number(arg)? / 2.0;
        }

        if let Some(arg) = arguments.get("d") {
            let r = self.module_argument_value_to_number(arg)? / 2.0;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("center") {
            center = self.module_argument_value_to_boolean(arg)?;
        }

        if center {
            todo!();
        }

        Some(Arc::new(Frustum::new(
            Vector3::new(0.0, 0.0, 0.0),
            height,
            radius1,
            radius2,
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26))),
        )))
    }

    fn create_translate(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        let mut offset = Vector3::new(0.0, 0.0, 0.0);

        let arguments = self.convert_args(&["v"], &instance.arguments);

        if let Some(arg) = arguments.get("v") {
            offset = self.module_argument_value_to_vector3(arg)?;
        }

        let translate = Translate::new(Arc::new(Group::from_list(&child_nodes)), offset);
        Some(Arc::new(translate))
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

    fn module_argument_value_to_number(&self, value: &ModuleArgumentValue) -> Option<f64> {
        match &value {
            ModuleArgumentValue::Number(value) => Some(*value),
            _ => todo!(),
        }
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
