use caustic_openscad::language_server::LanguageServerBackend;
use futures::StreamExt;
use tokio::sync::Mutex;
use tower;
use tower_lsp::LspService;
use tower_lsp::jsonrpc::Request;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmLspServer {
    service: Mutex<LspService<LanguageServerBackend>>,
}

#[wasm_bindgen]
impl WasmLspServer {
    #[wasm_bindgen(constructor)]
    pub fn new(output_callback: js_sys::Function) -> Self {
        let (service, mut messages) = LspService::new(|_| LanguageServerBackend::new());

        // Handle Outgoing (Rust -> JS)
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(msg) = messages.next().await {
                let json_str = serde_json::to_string(&msg).unwrap();
                let _ = output_callback.call1(&JsValue::NULL, &JsValue::from_str(&json_str));
            }
        });

        Self {
            service: Mutex::new(service),
        }
    }

    pub async fn notify_client_message(&self, msg: String) -> Result<Option<String>, String> {
        let mut service = self.service.lock().await;

        let request = serde_json::from_str::<Request>(&msg)
            .map_err(|err| format!("failed to parse message: {err:?}"))?;

        let response = tower::Service::<Request>::call(&mut *service, request)
            .await
            .map_err(|err| format!("call error: {:?}", err))?;

        if let Some(response) = response {
            let response_str = serde_json::to_string(&response)
                .map_err(|err| format!("Error encoding response: {err:?}"))?;

            Ok(Some(response_str))
        } else {
            Ok(None)
        }
    }
}
