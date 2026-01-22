mod hover;

use std::collections::HashMap;
use std::sync::Arc;

use caustic_core::utils::line_offset_to_position;
use tower_lsp::LanguageServer;
use tower_lsp::jsonrpc::{Error, ErrorCode, Result};
use tower_lsp::lsp_types::*;

use crate::parser::{StatementWithPosition, openscad_parse};
use crate::source::{Source, StringSource};
use crate::tokenizer::openscad_tokenize;

#[derive(Debug)]
pub struct LanguageServerBackend {
    document_map: tokio::sync::RwLock<HashMap<Url, String>>,
}

impl LanguageServerBackend {
    pub fn new() -> Self {
        Self {
            document_map: tokio::sync::RwLock::new(HashMap::new()),
        }
    }

    async fn parse_file(&self, url: &Url) -> Result<(String, Vec<StatementWithPosition>)> {
        let document_map = self.document_map.read().await;
        let text = match document_map.get(url) {
            Some(content) => content,
            None => {
                return Err(Error {
                    code: ErrorCode::InternalError,
                    message: format!("File not found: {url}").into(),
                    data: None,
                });
            } // Document not found
        };

        let source: Arc<Box<dyn Source>> = Arc::new(Box::new(StringSource::new(text)));
        let tokens = openscad_tokenize(source.clone())
            .tokens
            .ok_or_else(|| Error {
                code: ErrorCode::InternalError,
                message: format!("Failed to tokenize: {url}").into(),
                data: None,
            })?;

        let statements = openscad_parse(tokens, source)
            .statements
            .ok_or_else(|| Error {
                code: ErrorCode::InternalError,
                message: format!("Failed to parse: {url}").into(),
                data: None,
            })?;

        Ok((text.to_owned(), statements))
    }

    #[cfg(test)]
    pub async fn with_document(self, uri: Url, text: &str) -> Self {
        self.document_map.write().await.insert(uri, text.to_owned());
        self
    }
}

impl Default for LanguageServerBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LanguageServerBackend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        self.document_map.write().await.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        if let Some(change) = params.content_changes.into_iter().next() {
            self.document_map.write().await.insert(uri, change.text);
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let line = params.text_document_position_params.position.line as usize;
        let character = params.text_document_position_params.position.character as usize;

        let (code, statements) = self.parse_file(uri).await?;
        let pos = if let Some(pos) = line_offset_to_position(&code, line, character) {
            pos
        } else {
            return Ok(None);
        };

        self.handle_hover(statements, pos)
    }
}

#[cfg(test)]
pub mod test {
    use tower_lsp::lsp_types::{
        HoverContents, HoverParams, Position, TextDocumentIdentifier, TextDocumentPositionParams,
        Url, WorkDoneProgressParams,
    };

    use super::*;

    #[tokio::test]
    async fn test_hover_response() {
        let backend = LanguageServerBackend::new()
            .with_document(Url::parse("file:///test.scad").unwrap(), "circle(r=20);")
            .await;

        let params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier {
                    uri: Url::parse("file:///test.scad").unwrap(),
                },
                position: Position::new(0, 3),
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        };

        let hover = backend.hover(params).await.unwrap().unwrap();

        match hover.contents {
            HoverContents::Markup(MarkupContent { kind, value }) => {
                assert_eq!(kind, MarkupKind::Markdown);
                assert_eq!(value, "**Description:** Creates a circle at the origin");
            }
            _ => panic!("Expected scalar string"),
        }
    }
}
