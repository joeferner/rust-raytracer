use std::{collections::HashMap, rc::Rc, sync::Arc};

use rust_raytracer_core::{
    Camera, CameraBuilder, Color, Node, SceneData, Vector3,
    material::{Dielectric, Lambertian, Material, Metal},
    object::{
        BoundingVolumeHierarchy, BoxPrimitive, ConeFrustum, Group, Rotate, Scale, Sphere, Translate,
    },
};

use crate::interpreter::{Module, ModuleArgument, ModuleInstance, ModuleInstanceTree, Value};

struct Converter {
    camera: Option<Arc<Camera>>,
    world: Vec<Arc<dyn Node>>,
    lights: Vec<Arc<dyn Node>>,
    material_stack: Vec<Arc<dyn Material>>,
    variables: HashMap<String, Value>,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            camera: None,
            world: vec![],
            lights: vec![],
            material_stack: vec![],
            variables: HashMap::new(),
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

    fn current_material(&self) -> Arc<dyn Material> {
        if let Some(mat) = self.material_stack.last() {
            mat.clone()
        } else {
            Arc::new(Lambertian::new_from_color(Color::new(0.99, 0.85, 0.26)))
        }
    }

    fn process_module(&mut self, module: Rc<ModuleInstanceTree>) -> Option<Arc<dyn Node>> {
        if module.instance.module == Module::Color {
            let color = self.create_color(&module.instance)?;
            self.material_stack.push(color);
        } else if module.instance.module == Module::Lambertian {
            let color = self.create_lambertian(&module.instance)?;
            self.material_stack.push(color);
        } else if module.instance.module == Module::Dielectric {
            let color = self.create_dielectric(&module.instance)?;
            self.material_stack.push(color);
        } else if module.instance.module == Module::Metal {
            let color = self.create_metal(&module.instance)?;
            self.material_stack.push(color);
        } else if module.instance.module == Module::For {
            todo!("for");
        }

        let child_nodes = self.process_children(&module)?;

        match module.instance.module {
            Module::Cube => self.create_cube(&module.instance, child_nodes),
            Module::Sphere => self.create_sphere(&module.instance, child_nodes),
            Module::Cylinder => self.create_cylinder(&module.instance, child_nodes),
            Module::Translate => self.create_translate(&module.instance, child_nodes),
            Module::Rotate => self.create_rotate(&module.instance, child_nodes),
            Module::Scale => self.create_scale(&module.instance, child_nodes),
            Module::Camera => self.create_camera(&module.instance, child_nodes),
            Module::Color | Module::Lambertian | Module::Dielectric | Module::Metal => {
                self.material_stack.pop();
                Some(Arc::new(Group::from_list(&child_nodes)))
            }
            Module::For => todo!("for"),
            Module::Echo => self.evaluate_echo(&module.instance, child_nodes),
        }
    }

    fn process_children(&mut self, module: &Rc<ModuleInstanceTree>) -> Option<Vec<Arc<dyn Node>>> {
        let mut child_nodes: Vec<Arc<dyn Node>> = vec![];
        for child_module in module.children.borrow().iter() {
            if let Some(child_node) = self.process_module(child_module.clone()) {
                child_nodes.push(child_node);
            } else {
                return None;
            }
        }
        Some(child_nodes)
    }

    fn convert_args<'a>(
        &self,
        arg_names: &[&str],
        arguments: &'a [ModuleArgument],
    ) -> HashMap<String, &'a Value> {
        let mut results: HashMap<String, &'a Value> = HashMap::new();

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
            size = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("center") {
            center = arg.to_boolean()?;
        }

        let mut a = Vector3::new(0.0, 0.0, 0.0);
        let mut b = size;
        if center {
            a = a - (size / 2.0);
            b = b - (size / 2.0);
        }

        Some(Arc::new(BoxPrimitive::new(a, b, self.current_material())))
    }

    fn create_sphere(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!();
        }

        let mut radius = 1.0;

        let arguments = self.convert_args(&["r", "d"], &instance.arguments);

        if let Some(arg) = arguments.get("r") {
            radius = arg.to_number()?;
        } else if let Some(arg) = arguments.get("d") {
            radius = arg.to_number()? / 2.0;
        }

        Some(Arc::new(Sphere::new(
            Vector3::ZERO,
            radius,
            self.current_material(),
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
            height = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("r1") {
            radius1 = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("r2") {
            radius2 = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("r") {
            let r = arg.to_number()?;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("d1") {
            radius1 = arg.to_number()? / 2.0;
        }

        if let Some(arg) = arguments.get("d2") {
            radius2 = arg.to_number()? / 2.0;
        }

        if let Some(arg) = arguments.get("d") {
            let r = arg.to_number()? / 2.0;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("center") {
            center = arg.to_boolean()?;
        }

        let mut center_vec = Vector3::new(0.0, 0.0, 0.0);
        if center {
            center_vec.y -= height / 2.0;
        }

        Some(Arc::new(ConeFrustum::new(
            center_vec,
            height,
            radius1,
            radius2,
            self.current_material(),
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
            offset = arg.to_vector3()?;
        }

        let translate = Translate::new(Arc::new(Group::from_list(&child_nodes)), offset);
        Some(Arc::new(translate))
    }

    fn create_rotate(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        let arguments = self.convert_args(&["a", "v"], &instance.arguments);

        if let Some(arg) = arguments.get("a") {
            match arg {
                Value::Number(_deg_a) => todo!(),
                Value::Vector { items } => {
                    let a = Value::values_to_vector3(items)?;
                    let mut result: Arc<dyn Node> = Arc::new(Group::from_list(&child_nodes));
                    if a.x != 0.0 {
                        result = Arc::new(Rotate::rotate_x(result, a.x));
                    }
                    if a.y != 0.0 {
                        result = Arc::new(Rotate::rotate_y(result, a.y));
                    }
                    if a.z != 0.0 {
                        result = Arc::new(Rotate::rotate_z(result, a.z));
                    }
                    return Some(result);
                }
                _ => todo!("add error"),
            }
        }

        if let Some(_arg) = arguments.get("v") {
            todo!();
        }

        todo!();
    }

    fn create_scale(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        let arguments = self.convert_args(&["v"], &instance.arguments);

        if let Some(arg) = arguments.get("v") {
            let v = arg.to_vector3()?;
            let items_to_scale: Arc<dyn Node> = Arc::new(Group::from_list(&child_nodes));
            return Some(Arc::new(Scale::new(items_to_scale, v.x, v.y, v.z)));
        }

        todo!("missing arg");
    }

    fn create_color(&self, instance: &ModuleInstance) -> Option<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "alpha"], &instance.arguments);

        if let Some(arg) = arguments.get("alpha") {
            todo!("handle alpha {arg:?}");
        }

        if let Some(arg) = arguments.get("c") {
            let color = arg.to_color()?;
            return Some(Arc::new(Lambertian::new_from_color(color)));
        }

        todo!("missing arg");
    }

    fn create_lambertian(&self, instance: &ModuleInstance) -> Option<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "t"], &instance.arguments);

        if let Some(arg) = arguments.get("c") {
            let color = arg.to_color()?;
            return Some(Arc::new(Lambertian::new_from_color(color)));
        } else if let Some(arg) = arguments.get("t") {
            match arg {
                Value::Texture(texture) => Some(Arc::new(Lambertian::new(texture.clone()))),
                _ => todo!("unhandled {arg:?}"),
            }
        } else {
            todo!("missing arg");
        }
    }

    fn create_dielectric(&self, instance: &ModuleInstance) -> Option<Arc<dyn Material>> {
        let arguments = self.convert_args(&["n"], &instance.arguments);

        if let Some(arg) = arguments.get("n") {
            let refraction_index = arg.to_number()?;
            return Some(Arc::new(Dielectric::new(refraction_index)));
        } else {
            todo!("missing arg");
        }
    }

    fn create_metal(&self, instance: &ModuleInstance) -> Option<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "fuzz"], &instance.arguments);

        let mut color = Color::WHITE;
        let mut fuzz = 0.2;

        if let Some(arg) = arguments.get("c") {
            color = arg.to_color()?;
        }

        if let Some(arg) = arguments.get("fuzz") {
            fuzz = arg.to_number()?;
        }

        Some(Arc::new(Metal::new(color, fuzz)))
    }

    fn evaluate_echo(
        &self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!("should be empty");
        }

        let mut output = String::new();
        for (i, arg) in instance.arguments.iter().enumerate() {
            if i > 0 {
                output += ", ";
            }
            match arg {
                ModuleArgument::Positional(value) => output += &self.value_to_string(value),
                ModuleArgument::NamedArgument { name, value } => {
                    output += &format!("{name} = {}", self.value_to_string(value));
                }
            };
        }
        println!("{output}");

        None
    }

    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Number(number) => format!("{number}"),
            Value::Vector { items } => {
                let mut output = String::new();
                output += "[";
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        output += ", ";
                    }
                    output += &self.value_to_string(item);
                }
                output += "]";
                output
            }
            Value::True => "true".to_string(),
            Value::False => "false".to_string(),
            Value::Texture(texture) => todo!("texture {texture:?}"),
            Value::Range {
                start,
                end,
                increment,
            } => todo!("range: {start:?} {end:?} {increment:?}"),
            Value::Variable { name } => {
                if let Some(v) = self.variables.get(name) {
                    self.value_to_string(v)
                } else {
                    "undef".to_string()
                }
            }
        }
    }

    fn create_camera(
        &mut self,
        instance: &ModuleInstance,
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Option<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!("should be empty");
        }

        let arguments = self.convert_args(
            &[
                "image_width",
                "image_height",
                "samples_per_pixel",
                "max_depth",
                "vertical_fov",
                "look_from",
                "look_at",
                "up",
                "defocus_angle",
                "focus_distance",
                "background",
                "aspect_ratio",
            ],
            &instance.arguments,
        );

        let mut camera_builder = CameraBuilder::new();

        let mut seen_aspect_ratio = false;
        let mut seen_image_width = false;

        if let Some(arg) = arguments.get("aspect_ratio") {
            camera_builder.aspect_ratio = arg.to_number()?;
            seen_aspect_ratio = true;
        }

        if let Some(arg) = arguments.get("image_width") {
            camera_builder.image_width = arg.to_number()? as u32;
            seen_image_width = true;
        }

        if let Some(arg) = arguments.get("samples_per_pixel") {
            camera_builder.samples_per_pixel = arg.to_number()? as u32;
        }

        if let Some(arg) = arguments.get("max_depth") {
            camera_builder.max_depth = arg.to_number()? as u32;
        }

        if let Some(arg) = arguments.get("vertical_fov") {
            camera_builder.vertical_fov = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("defocus_angle") {
            camera_builder.defocus_angle = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("focus_distance") {
            camera_builder.focus_distance = arg.to_number()?;
        }

        if let Some(arg) = arguments.get("image_height") {
            let height = arg.to_number()?;
            if seen_image_width {
                camera_builder.aspect_ratio = camera_builder.image_width as f64 / height;
            } else if seen_aspect_ratio {
                camera_builder.image_width = (camera_builder.aspect_ratio * height) as u32;
            } else {
                camera_builder.aspect_ratio = 1.0;
                camera_builder.image_width = height as u32;
            }
        }

        if let Some(arg) = arguments.get("look_from") {
            camera_builder.look_from = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("look_at") {
            camera_builder.look_at = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("up") {
            camera_builder.up = arg.to_vector3()?;
        }

        if let Some(arg) = arguments.get("background") {
            camera_builder.background = arg.to_color()?;
        }

        self.camera = Some(Arc::new(camera_builder.build()));

        None
    }
}

pub fn openscad_convert(modules: Vec<Rc<ModuleInstanceTree>>) -> SceneData {
    let converter = Converter::new();
    converter.convert(modules)
}
