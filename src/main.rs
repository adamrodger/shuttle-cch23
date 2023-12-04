use axum::{http::StatusCode, routing::get, Router};

mod day01;
mod day04;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .nest("/1", day01::router())
        .nest("/4", day04::router());

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}
