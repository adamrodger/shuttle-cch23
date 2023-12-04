use axum::{extract::Json, routing::post, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/strength", post(strength))
        .route("/contest", post(contest))
}

#[derive(Debug, Clone, Deserialize)]
struct ReindeerStrengthRequest {
    strength: u32,
}

async fn strength(Json(body): Json<Vec<ReindeerStrengthRequest>>) -> String {
    body.iter().map(|r| r.strength).sum::<u32>().to_string()
}

#[derive(Debug, Clone, Deserialize)]
struct ReindeerContestRequest {
    name: String,
    speed: f32,
    height: u32,
    snow_magic_power: u32,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

#[derive(Debug, Clone, Serialize)]
struct ContestResponse {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

async fn contest(Json(body): Json<Vec<ReindeerContestRequest>>) -> Json<ContestResponse> {
    let mut fastest = &body[0];
    let mut tallest = &body[0];
    let mut magician = &body[0];
    let mut consumer = &body[0];

    for r in body.iter().skip(1) {
        if r.speed > fastest.speed {
            fastest = r;
        }

        if r.height > tallest.height {
            tallest = r;
        }

        if r.snow_magic_power > magician.snow_magic_power {
            magician = r;
        }

        if r.candies_eaten_yesterday > consumer.candies_eaten_yesterday {
            consumer = r;
        }
    }

    Json(ContestResponse {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.speed, fastest.name
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest.name, tallest.height
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician.name, magician.snow_magic_power
        ),
        consumer: format!("{} ate lots of candies, but also some grass", consumer.name),
    })
}
