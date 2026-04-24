use std::sync::Arc;

use salvo::{affix_state, prelude::*};
use salvo_oapi::{OpenApi, ToSchema, endpoint, extract::JsonBody, swagger_ui::SwaggerUi};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{AppState, TranslationRequest};

#[derive(Debug, Deserialize, ToSchema)]
pub struct JsonTranslateRequest {
    #[salvo(schema(example = "Hello world"))]
    pub q: String,
    #[salvo(schema(example = "en", nullable = true))]
    pub source: Option<String>,
    #[salvo(schema(example = "zh-CN"))]
    pub target: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JsonTranslateResponse {
    pub translated_text: String,
    pub detected_source: Option<String>,
    pub target: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub fn build_router(state: Arc<AppState>) -> Router {
    let mut api_router = Router::new()
        .push(Router::with_path("health").get(health))
        .push(Router::with_path("api/v1/translate").post(json_translate))
        .push(Router::with_path("translate_a/single").get(google_translate_compatible));

    let openapi = OpenApi::new("MyTranslator", "0.1.0").merge_router(&api_router);
    let doc = openapi.clone();

    api_router = api_router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json")
                .into_router("/swagger-ui"),
        );

    Router::new()
        .hoop(affix_state::inject(state))
        .push(api_router)
}

#[endpoint(tags("system"), responses((status_code = 200, body = HealthResponse)))]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

#[endpoint(
    tags("translate"),
    request_body = JsonTranslateRequest,
    responses(
        (status_code = 200, body = JsonTranslateResponse),
        (status_code = 400, description = "Invalid request"),
        (status_code = 500, description = "Backend error")
    )
)]
async fn json_translate(
    depot: &mut Depot,
    body: JsonBody<JsonTranslateRequest>,
) -> Result<Json<JsonTranslateResponse>, StatusError> {
    let body = body.into_inner();
    let request = TranslationRequest::new(body.q, body.source, body.target)
        .map_err(|error| error.into_status_error())?;
    let state = depot
        .obtain::<Arc<AppState>>()
        .map_err(|_| internal_state_error())?;
    let translated_text = state
        .translator
        .translate(&request)
        .await
        .map_err(|error| error.into_status_error())?;

    Ok(Json(JsonTranslateResponse {
        translated_text,
        detected_source: request.source,
        target: request.target,
    }))
}

#[handler]
async fn google_translate_compatible(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<Value>, StatusError> {
    let request = TranslationRequest::new(
        req.query::<String>("q").unwrap_or_default(),
        req.query::<String>("sl"),
        req.query::<String>("tl").unwrap_or_default(),
    )
    .map_err(|error| error.into_status_error())?;
    let state = depot
        .obtain::<Arc<AppState>>()
        .map_err(|_| internal_state_error())?;
    let translated_text = state
        .translator
        .translate(&request)
        .await
        .map_err(|error| error.into_status_error())?;

    Ok(Json(google_translate_payload(
        &translated_text,
        &request.text,
        request.source.as_deref().unwrap_or("auto"),
    )))
}

fn google_translate_payload(translated: &str, original: &str, detected_source: &str) -> Value {
    json!([
        [[translated, original, null, null, 1]],
        null,
        detected_source,
        null,
        null,
        null,
        null,
        []
    ])
}

fn internal_state_error() -> StatusError {
    StatusError::internal_server_error().brief("application state unavailable")
}
