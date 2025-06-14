use axum::{Router, extract::Path, routing::get};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::trace::TraceLayer;
use wow_mpq::Archive;

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
    tracing_subscriber::fmt::init();

    let archive = Arc::new(Mutex::new(Archive::open("patch.mpq")?));
    let archive_clone = archive.clone();
    let routes = Router::new().nest_service(
        "/assets/{path}",
        get(move |path| {
            let archive = archive_clone;
            get_wow_data(path, archive)
        }),
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, routes.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();

    Ok(())
}

async fn get_wow_data(Path(path): Path<String>, mpq: Arc<Mutex<Archive>>) -> String {
    let mut mpq = mpq.lock().unwrap();
    String::from_utf8(mpq.read_file(&path).unwrap()).unwrap()
}
