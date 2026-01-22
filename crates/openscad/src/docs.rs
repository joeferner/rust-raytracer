use crate::docs_builtin::BUILTIN_MODULE_DOCS;

pub struct ModuleDocs {
    pub description: String,
    pub arguments: Vec<ModuleDocsArguments>,
    pub examples: Vec<String>,
}

pub struct ModuleDocsArguments {
    pub name: String,
    pub description: String,
    pub default: Option<String>,
}

impl Clone for ModuleDocs {
    fn clone(&self) -> Self {
        ModuleDocs {
            description: self.description.clone(),
            arguments: self.arguments.clone(),
            examples: self.examples.clone(),
        }
    }
}

impl Clone for ModuleDocsArguments {
    fn clone(&self) -> Self {
        ModuleDocsArguments {
            name: self.name.clone(),
            description: self.description.clone(),
            default: self.default.clone(),
        }
    }
}

impl ModuleDocs {
    pub fn to_markdown(&self) -> String {
        let mut result = format!("**Description:** {}", self.description);

        if !self.arguments.is_empty() {
            result += "\n\n### Arguments:";
            for argument in &self.arguments {
                result += &format!("\n- `{}` {}", argument.name, argument.description);
                if let Some(default) = &argument.default {
                    result += &format!("Default: {default}");
                }
            }
        }

        if !self.examples.is_empty() {
            result += "\n\n### Examples:\n```";
            for example in &self.examples {
                result += &format!("\n{}", example);
            }
            result += "\n```";
        }

        result
    }
}

pub fn get_builtin_module_docs(module_id: &str) -> Option<ModuleDocs> {
    BUILTIN_MODULE_DOCS.get(module_id).cloned()
}
