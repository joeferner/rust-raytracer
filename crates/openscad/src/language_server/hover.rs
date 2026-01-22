use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

use crate::docs::get_builtin_module_docs;
use crate::language_server::LanguageServerBackend;
use crate::parser::{
    CallArgumentWithPosition, ModuleIdWithPosition, Statement, StatementWithPosition,
};

impl LanguageServerBackend {
    pub(super) fn handle_hover(
        &self,
        statements: Vec<StatementWithPosition>,
        pos: usize,
    ) -> Result<Option<Hover>> {
        for statement in statements {
            if statement.position.contains_pos(pos) {
                let result = match statement.item {
                    Statement::Empty => None,
                    Statement::Assignment {
                        identifier: _,
                        expr: _,
                    } => None,
                    Statement::Include { filename: _ } => None,
                    Statement::FunctionDecl {
                        function_name: _,
                        arguments: _,
                        expr: _,
                    } => None,
                    Statement::If {
                        expr: _,
                        true_statements: _,
                        false_statements: _,
                    } => None,
                    Statement::ModuleInstantiation {
                        module_id,
                        call_arguments,
                        child_statements,
                    } => self.hover_module_instantiation(
                        pos,
                        module_id,
                        call_arguments,
                        child_statements,
                    ),
                };
                if let Some(result) = result {
                    return Ok(Some(result));
                }
            }
        }

        Ok(None)
    }

    fn hover_module_instantiation(
        &self,
        pos: usize,
        module_id: ModuleIdWithPosition,
        call_arguments: Vec<CallArgumentWithPosition>,
        child_statements: Vec<StatementWithPosition>,
    ) -> Option<Hover> {
        if module_id.position.contains_pos(pos) {
            let module_docs = get_builtin_module_docs(&module_id.item);

            if let Some(module_docs) = module_docs {
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: module_docs.to_markdown(),
                    }),
                    range: None,
                });
            }
        }

        for call_argument in call_arguments {
            if call_argument.position.contains_pos(pos) {
                // TODO
            }
        }

        for child_statement in child_statements {
            if child_statement.position.contains_pos(pos) {
                // TODO
            }
        }

        None
    }
}
