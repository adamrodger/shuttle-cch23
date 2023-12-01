use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/*nums", get(cube_nums))
}

async fn cube_nums(Path(nums): Path<String>) -> impl IntoResponse {
    nums.split('/')
        .map(|n| n.parse::<i32>().unwrap_or(0))
        .reduce(|acc, n| acc ^ n)
        .unwrap_or(0)
        .pow(3)
        .to_string()
}
