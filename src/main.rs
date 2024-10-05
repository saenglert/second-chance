use axum::{
    routing::get,
    http::StatusCode,
    Json, Router
};
use values::{ISBNResult, ISBN};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod values;
pub mod momox;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(root));
}

async fn root() -> &'static str {
    "Hello World"
}

async fn isbns(
    Json(payload): Json<ISBN>,
) -> (StatusCode, Json<ISBNResult>) {
    let result = ISBNResult {
        isbn: payload.value(),
        name: "Test Book",
        price: 1.23
    };

    (StatusCode::OK, Json(result))
}