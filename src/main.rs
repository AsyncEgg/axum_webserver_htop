use axum::{routing::get, Router, Server, extract::State, Json, response::{IntoResponse, Html, Response}};
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(root_get))
    .route("/index.mjs", get(indexmjs_get))
    .route("/index.css", get(indexcss_get))
    .route("/api/cpus", get(cpus_get))
        .with_state(AppState {
            sys: Arc::new(Mutex::new(System::new()) )
        });

    let server = Server::bind(&"127.0.0.1:7474".parse().unwrap()).serve(app.into_make_service());
    
    let addr = server.local_addr();
    println!("Listening On {addr}");System::new();

    server.await.unwrap();
}
#[derive(Clone)]
struct AppState {
    sys: Arc<Mutex<System>>,
}

#[axum::debug_handler]
async fn root_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(markup) //Html(include_str!("index.html")) great for prod
}
#[axum::debug_handler]
async fn indexmjs_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.mjs").await.unwrap();
    Response::builder()
        .header("content-type", "application/javascript;charset=utf-8")
        .body(markup)
        .unwrap()
}

async fn indexcss_get() -> impl IntoResponse {
    let markup = tokio::fs::read_to_string("src/index.css").await.unwrap();
    Response::builder()
        .header("content-type", "text/css;charset=utf-8")
        .body(markup)
        .unwrap()
}

#[axum::debug_handler]
async fn cpus_get(State(state): State<AppState>) -> impl IntoResponse {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();

    let v: Vec<_> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    Json(v)
}
//http://127.0.0.1:7474/api/cpus