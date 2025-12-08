use anyhow::Result;
use opendal::{
    Operator,
    layers::{HttpClientLayer, LoggingLayer},
    raw::HttpClient,
    services::Ghac,
};
use reqwest::ClientBuilder;

pub fn build_ghac_operator(namespace: &str) -> Result<Operator> {
    let builder = Ghac::default().root(namespace);

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
