pub mod api;
pub mod config;
pub mod error;
pub mod llm;

use std::sync::Arc;

use llm::SharedTranslator;
use salvo::prelude::*;

use crate::{config::AppConfig, error::AppError, llm::OpenAiCompatibleTranslator};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub translator: SharedTranslator,
}

#[derive(Debug, Clone)]
pub struct TranslationRequest {
    pub text: String,
    pub source: Option<String>,
    pub target: String,
}

impl TranslationRequest {
    pub fn new(text: String, source: Option<String>, target: String) -> Result<Self, AppError> {
        if text.trim().is_empty() {
            return Err(AppError::InvalidRequest("`q` cannot be empty".to_owned()));
        }
        if target.trim().is_empty() {
            return Err(AppError::InvalidRequest(
                "`target`/`tl` cannot be empty".to_owned(),
            ));
        }

        Ok(Self {
            text,
            source: source.filter(|value| !value.trim().is_empty()),
            target,
        })
    }
}

pub fn build_state_from_config(config: AppConfig) -> Result<Arc<AppState>, AppError> {
    let token = config.llm.token()?;
    let translator = Arc::new(OpenAiCompatibleTranslator::new(config.llm.clone(), token)?);
    Ok(Arc::new(AppState { config, translator }))
}

pub async fn run(config_path: &str) -> Result<(), AppError> {
    let _ = dotenvy::dotenv();
    let config = AppConfig::load_from_file(config_path)?;
    let state = build_state_from_config(config.clone())?;
    let router = api::build_router(state);
    let address = format!("{}:{}", config.server.host, config.server.port);
    let acceptor = TcpListener::new(address).bind().await;

    Server::new(acceptor).serve(router).await;
    Ok(())
}
