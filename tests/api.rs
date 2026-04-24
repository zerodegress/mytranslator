use std::sync::Arc;

use mytranslator::{
    AppState, TranslationRequest,
    api::build_router,
    config::{AppConfig, LlmConfig, ServerConfig},
    llm::StubTranslator,
};
use salvo::{
    http::StatusCode,
    test::{ResponseExt, TestClient},
};
use serde_json::Value;

fn test_state() -> Arc<AppState> {
    Arc::new(AppState {
        config: AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_owned(),
                port: 5800,
            },
            llm: LlmConfig {
                base_url: "https://example.com/v1".to_owned(),
                model: "test-model".to_owned(),
                system_prompt: "translate".to_owned(),
                temperature: 0.0,
                timeout_secs: 10,
                token_env: "TOKEN".to_owned(),
            },
        },
        translator: Arc::new(StubTranslator {
            translated_text: "你好，世界".to_owned(),
        }),
    })
}

#[tokio::test]
async fn health_endpoint_returns_ok() {
    let router = build_router(test_state());
    let mut response = TestClient::get("http://127.0.0.1:5800/health")
        .send(router)
        .await;

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    let body = response.take_string().await.unwrap();
    assert!(body.contains("\"status\":\"ok\""));
}

#[tokio::test]
async fn swagger_ui_is_served() {
    let router = build_router(test_state());
    let mut response = TestClient::get("http://127.0.0.1:5800/swagger-ui/")
        .send(router)
        .await;

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    let body = response.take_string().await.unwrap();
    assert!(body.contains("Swagger UI"));
}

#[tokio::test]
async fn google_compatible_endpoint_returns_expected_shape() {
    let router = build_router(test_state());
    let mut response =
        TestClient::get("http://127.0.0.1:5800/translate_a/single?q=Hello&sl=en&tl=zh-CN")
            .send(router)
            .await;

    assert_eq!(response.status_code.unwrap(), StatusCode::OK);
    let body = response.take_json::<Value>().await.unwrap();
    assert_eq!(body[0][0][0], "你好，世界");
    assert_eq!(body[0][0][1], "Hello");
    assert_eq!(body[2], "en");
}

#[test]
fn translation_request_validates_input() {
    let error = TranslationRequest::new(String::new(), None, "zh-CN".to_owned()).unwrap_err();
    assert_eq!(error.to_string(), "invalid request: `q` cannot be empty");
}
