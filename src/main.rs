use axum::{extract::Path, http::StatusCode, routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(error))
        .route("/1/*nums", get(cube_many));

    Ok(router.into())
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn error() -> StatusCode {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn cube_many(Path(nums): Path<String>) -> String {
    nums.split('/')
        .map(|n| n.parse::<i32>().unwrap())
        .reduce(|acc, n| acc ^ n)
        .unwrap()
        .pow(3)
        .to_string()
}
