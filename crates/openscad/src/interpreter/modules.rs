use std::sync::Arc;

use caustic_core::{
    CameraBuilder, Color, Node, Vector3,
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    object::{BoxPrimitive, ConeFrustum, Group, Quad, Rotate, Scale, Sphere, Translate},
};

use crate::{
    Message, MessageLevel, Position, Result,
    interpreter::Interpreter,
    parser::{CallArgument, CallArgumentWithPosition, ModuleIdWithPosition, StatementWithPosition},
    value::Value,
};

impl Interpreter {
    pub(super) fn process_module_instantiation(
        &mut self,
        module_id: &ModuleIdWithPosition,
        arguments: &[CallArgumentWithPosition],
        child_statements: &[StatementWithPosition],
    ) -> Result<Vec<Arc<dyn Node>>> {
        let module_position = module_id.position.clone();

        if module_id.item == "color" {
            let m = self.create_color(arguments)?;
            self.material_stack.push(m);
        } else if module_id.item == "lambertian" {
            let m = self.create_lambertian(arguments)?;
            self.material_stack.push(m);
        } else if module_id.item == "dielectric" {
            let m = self.create_dielectric(arguments)?;
            self.material_stack.push(m);
        } else if module_id.item == "metal" {
            let m = self.create_metal(arguments)?;
            self.material_stack.push(m);
        } else if module_id.item == "diffuse_light" {
            let m = self.create_diffuse_light(arguments)?;
            self.material_stack.push(m);
        } else if module_id.item == "for" {
            return self.process_for_loop(arguments, child_statements);
        }

        let child_nodes = self.process_child_statements(child_statements)?;

        match module_id.item.as_str() {
            "cube" => self.create_cube(arguments, child_nodes).map(|n| vec![n]),
            "sphere" => self.create_sphere(arguments, child_nodes).map(|n| vec![n]),
            "cylinder" => self
                .create_cylinder(arguments, child_nodes)
                .map(|n| vec![n]),
            "quad" => self.create_quad(arguments, child_nodes).map(|n| vec![n]),
            "translate" => self
                .create_translate(arguments, child_nodes)
                .map(|n| vec![n]),
            "rotate" => self.create_rotate(arguments, child_nodes).map(|n| vec![n]),
            "scale" => self.create_scale(arguments, child_nodes).map(|n| vec![n]),
            "camera" => self.create_camera(arguments, child_nodes).map(|_| vec![]),
            "color" | "lambertian" | "dielectric" | "metal" | "diffuse_light" => {
                self.material_stack.pop();
                Ok(child_nodes)
            }
            "for" => panic!("already handled"),
            "echo" => self
                .evaluate_echo(arguments, child_nodes, module_position)
                .map(|_| vec![]),
            other => Err(Message {
                level: MessageLevel::Error,
                message: format!("unknown identifier \"{other}\""),
                position: module_id.position.clone(),
            }),
        }
    }

    fn create_cube(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!("should not have children");
        }

        let mut size = Vector3::new(0.0, 0.0, 0.0);
        let mut center = false;

        let arguments = self.convert_args(&["size", "center"], arguments)?;

        if let Some(arg) = arguments.get("size") {
            size = arg.item.to_vector3()?;
        }

        if let Some(arg) = arguments.get("center") {
            center = arg.item.to_boolean()?;
        }

        let mut a = Vector3::new(0.0, 0.0, 0.0);
        let mut b = size;
        if center {
            a = a - (size / 2.0);
            b = b - (size / 2.0);
        }

        Ok(Arc::new(BoxPrimitive::new(a, b, self.current_material())))
    }

    fn create_sphere(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!("should not have children");
        }

        let mut radius = 1.0;

        let arguments = self.convert_args(&["r", "d"], arguments)?;

        if let Some(arg) = arguments.get("r") {
            radius = arg.item.to_number()?;
        } else if let Some(arg) = arguments.get("d") {
            radius = arg.item.to_number()? / 2.0;
        }

        Ok(Arc::new(Sphere::new(
            Vector3::ZERO,
            radius,
            self.current_material(),
        )))
    }

    fn create_cylinder(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!("should not have children");
        }

        let mut height = 1.0;
        let mut radius1 = 1.0;
        let mut radius2 = 1.0;
        let mut center = false;

        let arguments = self.convert_args(
            &["h", "r1", "r2", "center", "r", "d", "d1", "d2"],
            arguments,
        )?;

        if let Some(arg) = arguments.get("h") {
            height = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("r1") {
            radius1 = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("r2") {
            radius2 = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("r") {
            let r = arg.item.to_number()?;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("d1") {
            radius1 = arg.item.to_number()? / 2.0;
        }

        if let Some(arg) = arguments.get("d2") {
            radius2 = arg.item.to_number()? / 2.0;
        }

        if let Some(arg) = arguments.get("d") {
            let r = arg.item.to_number()? / 2.0;
            radius1 = r;
            radius2 = r;
        }

        if let Some(arg) = arguments.get("center") {
            center = arg.item.to_boolean()?;
        }

        let mut center_vec = Vector3::new(0.0, 0.0, 0.0);
        if center {
            center_vec.y -= height / 2.0;
        }

        Ok(Arc::new(ConeFrustum::new(
            center_vec,
            height,
            radius1,
            radius2,
            self.current_material(),
        )))
    }

    fn create_quad(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if !child_nodes.is_empty() {
            todo!("should not have children");
        }

        let arguments = self.convert_args(&["q", "u", "v"], arguments)?;

        let q = if let Some(arg) = arguments.get("q") {
            arg.item.to_vector3()?
        } else {
            todo!("q is required");
        };

        let u = if let Some(arg) = arguments.get("u") {
            arg.item.to_vector3()?
        } else {
            todo!("q is required");
        };

        let v = if let Some(arg) = arguments.get("v") {
            arg.item.to_vector3()?
        } else {
            todo!("q is required");
        };

        Ok(Arc::new(Quad::new(q, u, v, self.current_material())))
    }

    fn create_translate(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child_nodes.is_empty() {
            todo!("should have children");
        }
        let child = Arc::new(Group::from_list(&child_nodes));

        let mut offset = Vector3::new(0.0, 0.0, 0.0);

        let arguments = self.convert_args(&["v"], arguments)?;

        if let Some(arg) = arguments.get("v") {
            offset = arg.item.to_vector3()?;
        }

        let translate = Translate::new(child, offset);
        Ok(Arc::new(translate))
    }

    fn create_rotate(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child_nodes.is_empty() {
            todo!("should have children");
        }
        let child = Arc::new(Group::from_list(&child_nodes));

        let arguments = self.convert_args(&["a", "v"], arguments)?;

        if let Some(arg) = arguments.get("a") {
            match &arg.item {
                Value::Number(_deg_a) => todo!(),
                Value::Vector { items } => {
                    let a = Value::values_to_vector3(items)?;
                    let mut result: Arc<dyn Node> = child;
                    if a.x != 0.0 {
                        result = Arc::new(Rotate::rotate_x(result, a.x));
                    }
                    if a.y != 0.0 {
                        result = Arc::new(Rotate::rotate_y(result, a.y));
                    }
                    if a.z != 0.0 {
                        result = Arc::new(Rotate::rotate_z(result, a.z));
                    }
                    return Ok(result);
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
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child_nodes.is_empty() {
            todo!("should have children");
        }
        let child = Arc::new(Group::from_list(&child_nodes));

        let arguments = self.convert_args(&["v"], arguments)?;

        if let Some(arg) = arguments.get("v") {
            let v = arg.item.to_vector3()?;
            return Ok(Arc::new(Scale::new(child, v.x, v.y, v.z)));
        }

        todo!("missing arg");
    }

    fn create_camera(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
    ) -> Result<()> {
        if !child_nodes.is_empty() {
            todo!("should not have children");
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
            arguments,
        )?;

        let mut camera_builder = CameraBuilder::new();

        let mut seen_aspect_ratio = false;
        let mut seen_image_width = false;

        if let Some(arg) = arguments.get("aspect_ratio") {
            camera_builder.aspect_ratio = arg.item.to_number()?;
            seen_aspect_ratio = true;
        }

        if let Some(arg) = arguments.get("image_width") {
            camera_builder.image_width = arg.item.to_number()? as u32;
            seen_image_width = true;
        }

        if let Some(arg) = arguments.get("samples_per_pixel") {
            camera_builder.samples_per_pixel = arg.item.to_number()? as u32;
        }

        if let Some(arg) = arguments.get("max_depth") {
            camera_builder.max_depth = arg.item.to_number()? as u32;
        }

        if let Some(arg) = arguments.get("vertical_fov") {
            camera_builder.vertical_fov = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("defocus_angle") {
            camera_builder.defocus_angle = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("focus_distance") {
            camera_builder.focus_distance = arg.item.to_number()?;
        }

        if let Some(arg) = arguments.get("image_height") {
            let height = arg.item.to_number()?;
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
            camera_builder.look_from = arg.item.to_vector3()?;
        }

        if let Some(arg) = arguments.get("look_at") {
            camera_builder.look_at = arg.item.to_vector3()?;
        }

        if let Some(arg) = arguments.get("up") {
            camera_builder.up = arg.item.to_vector3()?;
        }

        if let Some(arg) = arguments.get("background") {
            camera_builder.background = arg.item.to_color()?;
        }

        self.camera = Some(Arc::new(camera_builder.build()));

        Ok(())
    }

    fn evaluate_echo(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child_nodes: Vec<Arc<dyn Node>>,
        position: Position,
    ) -> Result<()> {
        if !child_nodes.is_empty() {
            todo!("should not have children");
        }

        let mut output = String::new();
        for (i, arg) in arguments.iter().enumerate() {
            if i > 0 {
                output += ", ";
            }
            match &arg.item {
                CallArgument::Expr { expr } => output += &self.expr_to_string(expr)?,
                CallArgument::NamedArgument { identifier, expr } => {
                    output += &format!("{identifier} = {}", self.expr_to_string(expr)?);
                }
            };
        }

        self.messages.push(Message {
            level: MessageLevel::Echo,
            message: output,
            position,
        });

        Ok(())
    }

    fn create_color(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "alpha"], arguments)?;

        if let Some(arg) = arguments.get("alpha") {
            todo!("handle alpha {arg:?}");
        }

        if let Some(arg) = arguments.get("c") {
            let color = arg.item.to_color()?;
            return Ok(Arc::new(Lambertian::new_from_color(color)));
        }

        todo!("missing arg");
    }

    fn create_lambertian(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "t"], arguments)?;

        if let Some(arg) = arguments.get("c") {
            let color = arg.item.to_color()?;
            Ok(Arc::new(Lambertian::new_from_color(color)))
        } else if let Some(arg) = arguments.get("t") {
            match &arg.item {
                Value::Texture(texture) => Ok(Arc::new(Lambertian::new(texture.clone()))),
                _ => todo!("unhandled {arg:?}"),
            }
        } else {
            todo!("missing arg");
        }
    }

    fn create_dielectric(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["n"], arguments)?;

        if let Some(arg) = arguments.get("n") {
            let refraction_index = arg.item.to_number()?;
            Ok(Arc::new(Dielectric::new(refraction_index)))
        } else {
            todo!("missing arg");
        }
    }

    fn create_metal(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c", "fuzz"], arguments)?;

        let mut color = Color::WHITE;
        let mut fuzz = 0.2;

        if let Some(arg) = arguments.get("c") {
            color = arg.item.to_color()?;
        }

        if let Some(arg) = arguments.get("fuzz") {
            fuzz = arg.item.to_number()?;
        }

        Ok(Arc::new(Metal::new(color, fuzz)))
    }

    fn create_diffuse_light(
        &mut self,
        arguments: &[CallArgumentWithPosition],
    ) -> Result<Arc<dyn Material>> {
        let arguments = self.convert_args(&["c"], arguments)?;

        let mut color = Color::WHITE;

        if let Some(arg) = arguments.get("c") {
            color = arg.item.to_color()?;
        }

        Ok(Arc::new(DiffuseLight::new_from_color(color)))
    }
}
