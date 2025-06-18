use axum::{
    Router,
    extract::Path,
    http::{Method, StatusCode},
    response::IntoResponse,
    routing::get,
};
use std::{
    fs,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use wow_mpq::PatchChain;

/*#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("../mangos-classic/src/game/Accounts/AccountMgr.h");
        //type AccountMgr;
        fn CreateAccount();
    }
}*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "INFO".into()),
        ))
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .with_writer(std::io::stdout)
                .with_target(false)
                .with_ansi(true)
                .with_line_number(false)
                .with_file(false),
        )
        .init();

    let mut chain = PatchChain::new();
    match fs::read_dir("Data") {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "MPQ" {
                            chain.add_archive(&path, 0)?
                        }
                    }
                }
            }
        }
        Err(err) => tracing::error!("{}", err),
    }
    let chain_mutex = Arc::new(Mutex::new(chain));
    let chain_clone = chain_mutex.clone();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    let routes = Router::new().route(
        "/assets/{*path}",
        get(move |path| {
            let chain = chain_clone;
            get_wow_data(path, chain)
        })
        .layer(cors),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();

    Ok(())
}

async fn get_wow_data(
    Path(path): Path<String>,
    mpq_files: Arc<Mutex<PatchChain>>,
) -> impl IntoResponse {
    let mut mpq_files = mpq_files.lock().unwrap();
    match mpq_files.read_file(&path) {
        Ok(file) => (StatusCode::OK, file),
        Err(err) => {
            tracing::error!("{}", err);
            (StatusCode::NOT_FOUND, Vec::new())
        }
    }
}
