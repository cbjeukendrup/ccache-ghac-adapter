use axum::{
    Router,
    body::Bytes,
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, head},
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root)).route(
        "/{*key}",
        head(ccache_head)
            .get(ccache_get)
            .put(ccache_put)
            .delete(ccache_delete),
    );

    let listener = tokio::net::TcpListener::bind("localhost:3459")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    "ccache GHAC adapter

Use HEAD, GET, PUT, DELETE on any path to interact with the cache."
}

async fn ccache_head(Path(path): Path<String>) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not found: {}", path))
}

async fn ccache_get(Path(path): Path<String>) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, format!("Not found: {}", path))
}

async fn ccache_put(Path(path): Path<String>, body: Bytes) -> impl IntoResponse {
    format!("Stored {} bytes at {}", body.len(), path)
}

async fn ccache_delete(Path(path): Path<String>) -> impl IntoResponse {
    format!("Deleted: {}", path)
}
