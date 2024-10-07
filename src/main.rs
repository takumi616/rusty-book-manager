use std::net::{Ipv4Addr, SocketAddr};

//anyhow::Error automatically converts error type implementing std::error::Error
//into anyhow::Error type.
use anyhow::Result;
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

//this macro is used to run main func on tokio runtime.
#[tokio::main]
async fn main() ->Result<()> {
    //Set up router
    let app = Router::new().route("/health", get(health_check));

    //Create listener with port 8080
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    //Run server
    Ok(axum::serve(listener, app).await?)
}
