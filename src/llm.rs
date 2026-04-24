use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{TranslationRequest, config::LlmConfig, error::AppError};

#[async_trait]
pub trait TranslatorBackend: Send + Sync {
    async fn translate(&self, request: &TranslationRequest) -> Result<String, AppError>;
}

pub type SharedTranslator = Arc<dyn TranslatorBackend>;

pub struct OpenAiCompatibleTranslator {
    client: Client,
    config: LlmConfig,
    token: String,
}

impl OpenAiCompatibleTranslator {
    pub fn new(config: LlmConfig, token: String) -> Result<Self, AppError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_secs))
            .build()
            .map_err(AppError::HttpClientBuild)?;

        Ok(Self {
            client,
            config,
            token,
        })
    }
}

#[async_trait]
impl TranslatorBackend for OpenAiCompatibleTranslator {
    async fn translate(&self, request: &TranslationRequest) -> Result<String, AppError> {
        let source = match request.source.as_deref() {
            Some("auto") | None | Some("") => "auto-detect".to_owned(),
            Some(lang) => lang.to_owned(),
        };

        let user_prompt = format!(
            "Translate the following text from {source} to {}.\nPreserve meaning, tone, formatting, line breaks, URLs, and placeholders.\nReturn only the translated text.\n\n{}",
            request.target, request.text
        );

        let payload = ChatCompletionsRequest {
            model: self.config.model.clone(),
            temperature: self.config.temperature,
            messages: vec![
                ChatMessage {
                    role: "system".to_owned(),
                    content: self.config.system_prompt.clone(),
                },
                ChatMessage {
                    role: "user".to_owned(),
                    content: user_prompt,
                },
            ],
        };

        let response = self
            .client
            .post(format!(
                "{}/chat/completions",
                self.config.base_url.trim_end_matches('/')
            ))
            .bearer_auth(&self.token)
            .json(&payload)
            .send()
            .await
            .map_err(AppError::UpstreamRequest)?
            .error_for_status()
            .map_err(AppError::UpstreamRequest)?;

        let body = response
            .json::<ChatCompletionsResponse>()
            .await
            .map_err(AppError::UpstreamRequest)?;

        body.choices
            .into_iter()
            .find_map(|choice| choice.message.content.into_text())
            .filter(|text| !text.trim().is_empty())
            .ok_or(AppError::UpstreamEmptyResponse)
    }
}

#[derive(Debug, Serialize)]
struct ChatCompletionsRequest {
    model: String,
    temperature: f32,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionsResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: AssistantMessage,
}

#[derive(Debug, Deserialize)]
struct AssistantMessage {
    content: AssistantContent,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum AssistantContent {
    Text(String),
    Parts(Vec<AssistantPart>),
}

impl AssistantContent {
    fn into_text(self) -> Option<String> {
        match self {
            Self::Text(text) => Some(text),
            Self::Parts(parts) => {
                let joined = parts
                    .into_iter()
                    .filter_map(|part| match part {
                        AssistantPart::Text { text, .. } => Some(text),
                        AssistantPart::Other => None,
                    })
                    .collect::<Vec<_>>()
                    .join("");
                if joined.is_empty() {
                    None
                } else {
                    Some(joined)
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum AssistantPart {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(other)]
    Other,
}

pub struct StubTranslator {
    pub translated_text: String,
}

#[async_trait]
impl TranslatorBackend for StubTranslator {
    async fn translate(&self, _request: &TranslationRequest) -> Result<String, AppError> {
        Ok(self.translated_text.clone())
    }
}
