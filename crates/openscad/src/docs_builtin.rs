use std::collections::HashMap;
use std::sync::LazyLock;

use crate::docs::{ModuleDocs, ModuleDocsArguments};

pub(crate) static BUILTIN_MODULE_DOCS: LazyLock<HashMap<&'static str, ModuleDocs>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    map.insert(
        "translate",
        ModuleDocs {
            description: "Translates (moves) its child elements along the specified vector."
                .to_owned(),
            arguments: vec![ModuleDocsArguments {
                name: "v".to_owned(),
                description: "vector to translate shape along".to_owned(),
                default: None,
            }],
            examples: vec!["translate(v = [x, y, z]) { ... }".to_owned()],
        },
    );

    map.insert(
        "circle",
        ModuleDocs {
            description: "Creates a circle at the origin. All parameters, except r, must be named."
                .to_owned(),
            arguments: vec![
                ModuleDocsArguments {
                    name: "r".to_owned(),
                    description: "circle radius. r name is the only one optional with circle."
                        .to_owned(),
                    default: None,
                },
                ModuleDocsArguments {
                    name: "d".to_owned(),
                    description: "circle diameter.".to_owned(),
                    default: None,
                },
            ],
            examples: vec![
                "circle(10);".to_owned(),
                "circle(r=10);".to_owned(),
                "circle(d=20);".to_owned(),
            ],
        },
    );

    map
});