use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

//this macro is used to run main func on tokio runtime.
#[tokio::main]
async fn main() {
    //Set up router
    let app = Router::new().route("/hello", get(hello_world));

    //Create listener with port 8080
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    //Run server
    axum::serve(listener, app).await.unwrap();
}
