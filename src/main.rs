use axum::{routing::get, Router, Server, extract::State};
use sysinfo::{CpuExt, System, SystemExt};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root_get))
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

async fn root_get() -> &'static str {
    "Ello"
}

async fn cpus_get(State(state): State<AppState>) -> String {
    use std::fmt::Write;

    let mut s = String::new();

    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu();


    for (index, cpu) in sys.cpus().iter().enumerate() {
        let usage = cpu.cpu_usage();
        writeln!(&mut s, "CPU {index} {usage}%").unwrap();
    }
    
    s
}