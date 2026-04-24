# MyTranslator

[中文](./README_ZH)

An API backend written in Rust that exposes an OpenAI-compatible LLM as a Google Translate-compatible service, built with Salvo.

## Warning

This project was written entirely by AI.

Do not assume it is production-safe without manual review. Validate security, correctness, rate limiting, error handling, prompt behavior, and compatibility with your target clients before deployment.

## Features

- Rust backend using `salvo` and `salvo-oapi`
- `config.toml` for runtime configuration
- `.env` for secret token injection
- Google Translate-compatible endpoint
- JSON translation endpoint for direct debugging
- Swagger UI and OpenAPI spec
- Unit and integration tests

## Endpoints

- `GET /health`
- `POST /api/v1/translate`
- `GET /translate_a/single?q=Hello&sl=en&tl=zh-CN`
- `GET /api-doc/openapi.json`
- `GET /swagger-ui/`

## Quick Start

```bash
cp .env.example .env
```

Set your token in `.env`:

```env
TOKEN=your-openai-compatible-token
```

Run locally:

```bash
cargo run
```

Then open:

- Swagger UI: `http://127.0.0.1:5800/swagger-ui/`
- Health check: `http://127.0.0.1:5800/health`

## Configuration

`config.toml` controls the server and upstream LLM settings:

```toml
[server]
host = "127.0.0.1"
port = 5800

[llm]
base_url = "https://api.openai.com/v1"
model = "gpt-4o-mini"
system_prompt = "You are a translation engine..."
temperature = 0.0
timeout_secs = 60
token_env = "TOKEN"
```

## API Examples

JSON translation:

```bash
curl -X POST http://127.0.0.1:5800/api/v1/translate \
  -H 'Content-Type: application/json' \
  -d '{"q":"Hello world","source":"en","target":"zh-CN"}'
```

Google-compatible translation:

```bash
curl 'http://127.0.0.1:5800/translate_a/single?q=Hello&sl=en&tl=zh-CN'
```

## Development

```bash
cargo fmt --all
cargo test
```

Source layout:

- `src/api.rs`: routes and handlers
- `src/config.rs`: TOML config loading
- `src/llm.rs`: upstream LLM adapter
- `tests/api.rs`: endpoint tests
