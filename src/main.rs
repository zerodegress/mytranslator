#[tokio::main]
async fn main() -> Result<(), mytranslator::error::AppError> {
    mytranslator::run("config.toml").await
}
