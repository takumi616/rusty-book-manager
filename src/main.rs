use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

//Handler
async fn hello_world() -> &'static str {
    "Hello, world!"
}

//This macro is used to be able to run main() programs on tokio asynchronous runtime.
#[tokio::main]
async fn main() {
    //Initializes a router and register a handler to specified path.
    let app = Router::new().route("/hello", get(hello_world));
    //Creates a new socket address from localhost and port 8080.
    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    //Creates a new TcpListener with specified address.
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);

    //Runs server.
    axum::serve(listener, app).await.unwrap();
}