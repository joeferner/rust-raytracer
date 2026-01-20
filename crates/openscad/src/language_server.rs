use std::collections::HashMap;

use tower_lsp::LanguageServer;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

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

    #[cfg(test)]
    pub async fn with_document(self, uri: Url, text: &str) -> Self {
        self.document_map.write().await.insert(uri, text.to_owned());
        self
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

    async fn hover(&self, _: HoverParams) -> Result<Option<Hover>> {
        Ok(Some(Hover {
            contents: HoverContents::Scalar(MarkedString::String("Hello from WASM!".to_string())),
            range: None,
        }))
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
                    uri: Url::parse("file:///test.rs").unwrap(),
                },
                position: Position::new(5, 10),
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        };

        let hover = backend.hover(params).await.unwrap().unwrap();

        match hover.contents {
            HoverContents::Scalar(MarkedString::String(s)) => {
                assert_eq!(s, "Hello from WASM!");
            }
            _ => panic!("Expected scalar string"),
        }
    }
}
