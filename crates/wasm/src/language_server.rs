use caustic_openscad::language_server::LanguageServerBackend;
use futures::StreamExt;
use tokio::sync::Mutex;
use tower;
use tower_lsp::LspService;
use tower_lsp::jsonrpc::{Id, Request};
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub struct WasmLspServer {
    service: Mutex<LspService<LanguageServerBackend>>,
}

#[wasm_bindgen]
impl WasmLspServer {
    #[wasm_bindgen(constructor)]
    pub fn new(output_callback: js_sys::Function) -> Self {
        let (service, mut messages) = LspService::new(LanguageServerBackend::new);

        // Handle Outgoing (Rust -> JS)
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(msg) = messages.next().await {
                console::log_1(&format!("rust Rust -> JS {msg}").into());
                let json_str = serde_json::to_string(&msg).unwrap();
                let _ = output_callback.call1(&JsValue::NULL, &JsValue::from_str(&json_str));
            }
        });

        Self {
            service: Mutex::new(service),
        }
    }

    pub async fn initialize(&self, params: String) -> String {
        console::log_1(&"rust: manual initialize call".into());

        // 1. Parse the incoming params (or construct manually)
        let params: serde_json::Value =
            serde_json::from_str(&params).unwrap_or(serde_json::json!({}));

        // 2. Build a JSON-RPC request for "initialize"
        // tower-lsp expects this specific method to start the lifecycle
        let request = Request::build("initialize")
            .id(Id::Number(1))
            .params(params)
            .finish();

        let mut service = self.service.lock().await;

        // 3. Call the service just like you do in notify_client_message
        match tower::Service::<Request>::call(&mut *service, request).await {
            Ok(Some(response)) => serde_json::to_string(&response)
                .unwrap_or_else(|_| "Error encoding response".into()),
            Ok(None) => "null".to_string(),
            Err(e) => format!("Error: {:?}", e),
        }
    }

    pub async fn notify_client_message(&self, msg: String) {
        console::log_1(&format!("rust notify_client_message {msg}").into());
        let mut service = self.service.lock().await;
        if let Ok(req) = serde_json::from_str::<Request>(&msg) {
            let r = tower::Service::<Request>::call(&mut *service, req).await;
            match r {
                Ok(ok) => {
                    console::log_1(&format!("rust notify_client_message call ok {ok:?}").into())
                }
                Err(err) => {
                    console::log_1(&format!("rust notify_client_message call err {err:?}").into())
                }
            }
        }
    }
}
