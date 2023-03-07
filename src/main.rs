use axum::{routing::get, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_get));

    let server = Server::bind(&"127.0.0.1:7474".parse().unwrap()).serve(app.into_make_service());
    
    let addr = server.local_addr();
    println!("Listening On {addr}");
    
    server.await.unwrap();
}

async fn root_get() -> &'static str {
    "Hi from Axum"
}
