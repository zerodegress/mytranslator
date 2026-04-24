# Repository Guidelines

## Project Structure & Module Organization

This repository contains a Rust API backend that exposes an LLM through Google Translate compatible endpoints.

- `src/main.rs`: process entrypoint; loads config and starts the Salvo server.
- `src/lib.rs`: shared application state and startup wiring.
- `src/api.rs`: HTTP routes, handlers, and Swagger/OpenAPI integration.
- `src/config.rs`: `config.toml` parsing and environment-based token loading.
- `src/llm.rs`: OpenAI-compatible upstream client and translation adapter.
- `src/error.rs`: application error types and HTTP error mapping.
- `tests/api.rs`: integration tests for health, translation, and Swagger UI.
- `config.toml`: local runtime configuration.
- `.env.example`: required environment variable template.

## Build, Test, and Development Commands

- `cargo run`: start the server with `config.toml` in the repository root.
- `cargo test`: run unit tests, integration tests, and doc tests.
- `cargo fmt --all`: format all Rust code before review.
- `cargo check`: fast compile check without producing a release binary.
- `cargo build --release`: build an optimized production binary.

Example local setup:

```bash
cp .env.example .env
cargo run
```

## Coding Style & Naming Conventions

Use standard Rust formatting with `cargo fmt`; do not hand-format around it. Prefer 4-space indentation, `snake_case` for functions/modules/files, `CamelCase` for structs/enums/traits, and clear domain names such as `TranslationRequest` or `OpenAiCompatibleTranslator`.

Keep modules focused: routing in `api.rs`, config in `config.rs`, upstream logic in `llm.rs`. Prefer explicit error propagation with `Result` and `thiserror` instead of panics in application code.

## Testing Guidelines

Tests use Rust’s built-in test framework with `tokio::test` for async handlers. Add integration tests under `tests/` for HTTP behavior and unit tests next to implementation when the logic is isolated.

Name tests by behavior, for example `health_endpoint_returns_ok` or `translation_request_validates_input`. New endpoints should include at least one success-path test and one invalid-input test.

## Commit & Pull Request Guidelines

There is no existing commit history yet, so use short imperative commit messages such as `Add batch translation endpoint` or `Fix Swagger route redirect`. Keep each commit scoped to one logical change.

Pull requests should include:

- A brief summary of behavior changes.
- Config or environment changes, if any.
- Test evidence, usually `cargo test`.
- Example request/response snippets when API behavior changes.

## Security & Configuration Tips

Never commit real tokens. Keep secrets in `.env` and reserve `config.toml` for non-secret settings such as host, port, model, and upstream base URL.
