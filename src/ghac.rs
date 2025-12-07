use anyhow::Result;
use opendal::{
    Operator,
    layers::{HttpClientLayer, LoggingLayer},
    raw::HttpClient,
    services::Ghac,
};
use reqwest::ClientBuilder;

use crate::VERSION;

pub fn build_ghac_operator(namespace: &str) -> Result<Operator> {
    let mut builder = Ghac::default().root("/ccache");

    builder = if namespace.is_empty() {
        builder.version(&format!("sccache-v{VERSION}"))
    } else {
        builder.version(&format!("sccache-v{VERSION}-{namespace}"))
    };

    let op = Operator::new(builder)?
        .layer(HttpClientLayer::new(set_user_agent()))
        .layer(LoggingLayer::default())
        .finish();
    Ok(op)
}

pub fn set_user_agent() -> HttpClient {
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let client = ClientBuilder::new().user_agent(user_agent).build().unwrap();
    HttpClient::with(client)
}
