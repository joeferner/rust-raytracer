pub struct ModuleDocs {
    pub description: String,
    pub arguments: Vec<ModuleDocsArguments>,
    pub examples: Vec<String>,
}

pub struct ModuleDocsArguments {
    name: String,
    description: String,
    default: Option<String>,
}

impl ModuleDocs {
    pub fn to_markdown(&self) -> String {
        let mut result = format!("**Description:** {}", self.description);

        if !self.arguments.is_empty() {
            result += "\n\n**Arguments:**";
            for argument in &self.arguments {
                result += &format!("\n- `{}` {}", argument.name, argument.description);
                if let Some(default) = &argument.default {
                    result += &format!("Default: {default}");
                }
            }
        }

        if !self.examples.is_empty() {
            result += "\n\n**Examples:**\n```";
            for example in &self.examples {
                result += &format!("\n{}", example);
            }
            result += "\n```";
        }

        result
    }
}

pub fn get_builtin_module_docs(module_id: &str) -> Option<ModuleDocs> {
    match module_id {
        "translate" => Some(ModuleDocs {
            description: "Translates (moves) its child elements along the specified vector."
                .to_owned(),
            arguments: vec![ModuleDocsArguments {
                name: "v".to_owned(),
                description: "vector to translate shape along".to_owned(),
                default: None,
            }],
            examples: vec!["translate(v = [x, y, z]) { ... }".to_owned()],
        }),
        "circle" => Some(ModuleDocs {
            description: "Creates a circle at the origin".to_owned(),
            arguments: vec![],
            examples: vec![],
        }),
        _ => None,
    }
}
