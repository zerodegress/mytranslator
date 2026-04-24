use salvo::http::StatusError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("failed to read config file `{path}`")]
    ConfigRead {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to parse config.toml")]
    ConfigParse(#[from] toml::de::Error),
    #[error("missing token environment variable `{env_name}`")]
    MissingTokenEnv { env_name: String },
    #[error("http client init failed")]
    HttpClientBuild(#[source] reqwest::Error),
    #[error("upstream request failed")]
    UpstreamRequest(#[source] reqwest::Error),
    #[error("upstream returned no translation text")]
    UpstreamEmptyResponse,
    #[error("invalid request: {0}")]
    InvalidRequest(String),
}

impl AppError {
    pub fn into_status_error(&self) -> StatusError {
        match self {
            Self::InvalidRequest(message) => StatusError::bad_request().brief(message.clone()),
            Self::MissingTokenEnv { env_name } => StatusError::internal_server_error()
                .brief(format!("missing environment variable `{env_name}`")),
            _ => StatusError::internal_server_error().brief(self.to_string()),
        }
    }
}
