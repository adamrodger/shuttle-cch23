use axum::{routing::post, Json, Router};
use serde::Serialize;

pub fn router() -> Router {
    Router::new().route("/", post(elf_count))
}

#[derive(Clone, Copy, Debug, Serialize)]
struct ElfCountResponse {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

async fn elf_count(body: String) -> Json<ElfCountResponse> {
    let elves = body.matches("elf").count();
    let shelves = body.matches("shelf").count();
    let elves_on_shelves = body.matches("elf on a shelf").count();

    Json(ElfCountResponse {
        elf: elves,
        elf_on_a_shelf: elves_on_shelves,
        shelf_with_no_elf_on_it: shelves - elves_on_shelves,
    })
}
