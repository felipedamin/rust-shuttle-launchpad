use axum::{response::IntoResponse, routing::get, Router};
// use std::net::SocketAddr;

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/health", get(health_check));
//     let addr = SocketAddr::new([0,0,0,0].into(), 3000);

//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

#[shuttle_runtime::main]
async fn shuttle() -> shuttle_axum::ShuttleAxum {
    let app = Router::new().route("/", get(health_check));
    Ok(app.into())
}


async fn health_check() -> impl IntoResponse {
    "This Service Is Healthy!"
}
