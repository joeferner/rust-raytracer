// use std::{collections::HashMap, rc::Rc, sync::Arc};

// use rust_raytracer_core::{
//     Camera, CameraBuilder, Color, Node, SceneData, Vector3,
//     material::{Dielectric, Lambertian, Material, Metal},
//     object::{
//         BoundingVolumeHierarchy, BoxPrimitive, ConeFrustum, Group, Rotate, Scale, Sphere, Translate,
//     },
// };

// use crate::interpreter::{Module, ModuleArgument, ModuleInstance, ModuleInstanceTree, Value};

// impl Converter {

//     fn process_children(&mut self, module: &Rc<ModuleInstanceTree>) -> Option<Vec<Arc<dyn Node>>> {
//         let mut child_nodes: Vec<Arc<dyn Node>> = vec![];
//         for child_module in module.children.borrow().iter() {
//             if let Some(child_node) = self.process_module(child_module.clone()) {
//                 child_nodes.push(child_node);
//             } else {
//                 return None;
//             }
//         }
//         Some(child_nodes)
//     }

//     fn convert_args<'a>(
//         &self,
//         arg_names: &[&str],
//         arguments: &'a [ModuleArgument],
//     ) -> HashMap<String, &'a Value> {
//         let mut results: HashMap<String, &'a Value> = HashMap::new();

//         let mut found_named_arg = false;
//         for (pos, arg) in arguments.iter().enumerate() {
//             match arg {
//                 ModuleArgument::Positional(value) => {
//                     if found_named_arg {
//                         todo!("add error, no positional args after named arg");
//                     }
//                     if let Some(arg_name) = arg_names.get(pos) {
//                         results.insert(arg_name.to_string(), value);
//                     } else {
//                         todo!("arg past end of list");
//                     }
//                 }
//                 ModuleArgument::NamedArgument { name, value } => {
//                     found_named_arg = true;
//                     if arg_names.contains(&name.as_str()) {
//                         results.insert(name.to_string(), value);
//                     } else {
//                         todo!("unknown arg name");
//                     }
//                 }
//             }
//         }

//         results
//     }

// }
