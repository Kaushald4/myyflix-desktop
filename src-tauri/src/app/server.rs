use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::routes;

pub fn start() {
    tauri::async_runtime::spawn(async {
        let app = Router::new()
            .merge(routes::health::router())
            .merge(routes::extract::router())
            .merge(routes::stream::router());

        let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
        let listener = TcpListener::bind(addr).await.unwrap();

        axum::serve(listener, app).await.unwrap();
    });
}
