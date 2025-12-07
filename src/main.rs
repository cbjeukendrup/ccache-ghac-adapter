use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, head},
};

mod ghac;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    let namespace = std::env::var("CCACHE_GHAC_NAMESPACE").unwrap_or_default();
    let operator = ghac::build_ghac_operator(&namespace).unwrap();

    operator.check().await.unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/{*key}",
            head(ccache_head)
                .get(ccache_get)
                .put(ccache_put)
                .delete(ccache_delete),
        )
        .with_state(operator);

    let listener = tokio::net::TcpListener::bind("localhost:3459")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    "ccache GHAC adapter

Use HEAD, GET, PUT, DELETE on any path to interact with the cache."
}

async fn ccache_head(
    State(operator): State<opendal::Operator>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match operator.exists(&path).await {
        Ok(true) => StatusCode::OK,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn ccache_get(
    State(operator): State<opendal::Operator>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match operator.read(&path).await {
        Ok(data) => (StatusCode::OK, data.to_bytes()),
        Err(e) if e.kind() == opendal::ErrorKind::NotFound => (StatusCode::NOT_FOUND, Bytes::new()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Bytes::new()),
    }
}

async fn ccache_put(
    State(operator): State<opendal::Operator>,
    Path(path): Path<String>,
    body: Bytes,
) -> impl IntoResponse {
    match operator.write(&path, body).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn ccache_delete(
    State(operator): State<opendal::Operator>,
    Path(path): Path<String>,
) -> impl IntoResponse {
    match operator.delete(&path).await {
        Ok(_) => StatusCode::OK,
        Err(e) if e.kind() == opendal::ErrorKind::NotFound => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
