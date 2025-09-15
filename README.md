# 🚀 Rust Actix Web API with OpenAPI/Swagger

This is a simple REST API built with [Actix Web](https://actix.rs/), documented using [utoipa](https://docs.rs/utoipa) and served with [Swagger UI](https://docs.rs/utoipa-swagger-ui).

It demonstrates how to:
- Define HTTP routes with Actix Web
- Automatically generate OpenAPI documentation
- Serve interactive Swagger UI
- Use environment variables for API metadata

---

## 📦 Features

- **Hello World** endpoint (`GET /`)
- **Echo** endpoint (`POST /echo`)
- **Manual greeting** endpoint (`GET /hey`)
- Auto-generated **OpenAPI JSON spec** at `/docs/openapi.json`
- **Swagger UI** at `/swagger/` for interactive API exploration
- Configurable API metadata (title, contact info) via environment variables

---

## 🛠️ Getting Started

### Prerequisites
- Rust (latest stable recommended) → [Install Rust](https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust)

### Running the API

```bash
cargo watch -x run