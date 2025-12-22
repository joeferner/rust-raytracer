use std::sync::Arc;

use rust_raytracer_core::{
    CameraBuilder, Node, Vector3,
    object::{BoxPrimitive, ConeFrustum, Rotate, Scale, Sphere, Translate},
};

use crate::{
    interpreter::{Interpreter, Result},
    parser::{
        CallArgument, CallArgumentWithPosition, ModuleId, ModuleIdWithPosition,
        StatementWithPosition,
    },
    value::Value,
};

impl Interpreter {
    pub(super) fn process_module_instantiation(
        &mut self,
        module_id: &ModuleIdWithPosition,
        arguments: &[CallArgumentWithPosition],
        child_statements: &[StatementWithPosition],
    ) -> Result<Option<Arc<dyn Node>>> {
        if module_id.item == ModuleId::Color {
            let color = self.create_color(arguments)?;
            self.material_stack.push(color);
        } else if module_id.item == ModuleId::Lambertian {
            let color = self.create_lambertian(arguments)?;
            self.material_stack.push(color);
        } else if module_id.item == ModuleId::Dielectric {
            let color = self.create_dielectric(arguments)?;
            self.material_stack.push(color);
        } else if module_id.item == ModuleId::Metal {
            let color = self.create_metal(arguments)?;
            self.material_stack.push(color);
        } else if module_id.item == ModuleId::For {
            return self.process_for_loop(arguments, child_statements);
        }

        let child = self.process_child_statements(child_statements)?;

        match &module_id.item {
            ModuleId::Cube => self.create_cube(arguments, child).map(Some),
            ModuleId::Sphere => self.create_sphere(arguments, child).map(Some),
            ModuleId::Cylinder => self.create_cylinder(arguments, child).map(Some),
            ModuleId::Translate => self.create_translate(arguments, child).map(Some),
            ModuleId::Rotate => self.create_rotate(arguments, child).map(Some),
            ModuleId::Scale => self.create_scale(arguments, child).map(Some),
            ModuleId::Camera => self.create_camera(arguments, child).map(|_| None),
            ModuleId::Color | ModuleId::Lambertian | ModuleId::Dielectric | ModuleId::Metal => {
                self.material_stack.pop();
                Ok(child)
            }
            ModuleId::For => panic!("already handled"),
            ModuleId::Echo => self.evaluate_echo(arguments, child).map(|_| None),
            ModuleId::Identifier(identifier) => {
                todo!("ModuleId::Identifier {identifier}")
            }
        }
    }

    fn create_cube(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child.is_some() {
            todo!("should not have children");
        }

        let mut size = Vector3::new(0.0, 0.0, 0.0);
        let mut center = false;

        let arguments = self.convert_args(&["size", "center"], arguments)?;

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

        Ok(Arc::new(BoxPrimitive::new(a, b, self.current_material())))
    }

    fn create_sphere(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child.is_some() {
            todo!("should not have children");
        }

        let mut radius = 1.0;

        let arguments = self.convert_args(&["r", "d"], arguments)?;

        if let Some(arg) = arguments.get("r") {
            radius = arg.to_number()?;
        } else if let Some(arg) = arguments.get("d") {
            radius = arg.to_number()? / 2.0;
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
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        if child.is_some() {
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

        Ok(Arc::new(ConeFrustum::new(
            center_vec,
            height,
            radius1,
            radius2,
            self.current_material(),
        )))
    }

    fn create_translate(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        let child = child.unwrap_or_else(|| todo!("should have children"));

        let mut offset = Vector3::new(0.0, 0.0, 0.0);

        let arguments = self.convert_args(&["v"], arguments)?;

        if let Some(arg) = arguments.get("v") {
            offset = arg.to_vector3()?;
        }

        let translate = Translate::new(child, offset);
        Ok(Arc::new(translate))
    }

    fn create_rotate(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        let child = child.unwrap_or_else(|| todo!("should have children"));

        let arguments = self.convert_args(&["a", "v"], arguments)?;

        if let Some(arg) = arguments.get("a") {
            match arg {
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
        child: Option<Arc<dyn Node>>,
    ) -> Result<Arc<dyn Node>> {
        let child = child.unwrap_or_else(|| todo!("should have children"));

        let arguments = self.convert_args(&["v"], arguments)?;

        if let Some(arg) = arguments.get("v") {
            let v = arg.to_vector3()?;
            return Ok(Arc::new(Scale::new(child, v.x, v.y, v.z)));
        }

        todo!("missing arg");
    }

    fn create_camera(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<()> {
        if child.is_some() {
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

        Ok(())
    }

    fn evaluate_echo(
        &mut self,
        arguments: &[CallArgumentWithPosition],
        child: Option<Arc<dyn Node>>,
    ) -> Result<()> {
        if child.is_some() {
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

        self.output += &format!("{output}\n");

        Ok(())
    }
}
