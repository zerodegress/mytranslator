# MyTranslator

这是一个使用 Rust 编写的 API 后端项目，通过 OpenAI 兼容格式的 LLM 提供 Google Translate 兼容接口，底层框架使用 Salvo。

## 警告

这个项目完全由 AI 编写。

请不要在未人工审查的情况下直接将其视为可安全投入生产。上线前应自行验证安全性、正确性、限流、错误处理、提示词行为以及与目标客户端的兼容性。

## 功能

- 基于 `salvo` 和 `salvo-oapi` 的 Rust 后端
- 使用 `config.toml` 管理运行配置
- 使用 `.env` 注入密钥
- 提供 Google Translate 兼容接口
- 提供 JSON 翻译接口，便于调试
- 提供 Swagger UI 和 OpenAPI 文档
- 包含单元测试与集成测试

## 接口

- `GET /health`
- `POST /api/v1/translate`
- `GET /translate_a/single?q=Hello&sl=en&tl=zh-CN`
- `GET /api-doc/openapi.json`
- `GET /swagger-ui/`

## 快速开始

先复制环境变量模板：

```bash
cp .env.example .env
```

在 `.env` 中填入你的 token：

```env
TOKEN=your-openai-compatible-token
```

启动服务：

```bash
cargo run
```

启动后可访问：

- Swagger UI：`http://127.0.0.1:5800/swagger-ui/`
- 健康检查：`http://127.0.0.1:5800/health`

## 配置

`config.toml` 用于控制服务地址和上游 LLM 参数：

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

## 调用示例

JSON 翻译接口：

```bash
curl -X POST http://127.0.0.1:5800/api/v1/translate \
  -H 'Content-Type: application/json' \
  -d '{"q":"Hello world","source":"en","target":"zh-CN"}'
```

Google 兼容接口：

```bash
curl 'http://127.0.0.1:5800/translate_a/single?q=Hello&sl=en&tl=zh-CN'
```

## 开发

```bash
cargo fmt --all
cargo test
```

源码结构：

- `src/api.rs`：路由与处理器
- `src/config.rs`：TOML 配置加载
- `src/llm.rs`：上游 LLM 适配层
- `tests/api.rs`：接口测试
