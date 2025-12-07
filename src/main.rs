use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, head},
};
use clap::Parser;

mod ghac;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "ccache-ghac-adapter")]
#[command(version = VERSION)]
#[command(about = "ccache adapter for GitHub Actions Cache", long_about = None)]
struct Cli {
    /// Port to listen on
    #[arg(short, long, default_value_t = 3459)]
    port: u16,

    /// Namespace for the cache
    #[arg(short, long)]
    namespace: Option<String>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let namespace = cli.namespace.unwrap_or_default();
    let operator = ghac::build_ghac_operator(&namespace).unwrap();

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

    let addr = format!("localhost:{}", cli.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
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
