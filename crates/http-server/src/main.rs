use axum::{
    body::{Bytes, Full},
    extract::Path,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;

use advent_of_code::solve_raw;

#[tokio::main]
async fn main() {
    #![allow(clippy::unwrap_used)]

    let app = Router::new()
        .route("/", get(handle_get))
        .route("/solve/:year/:day/:part", post(handle_post));

    println!("Running on port 8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_get() -> Response<Full<Bytes>> {
    #![allow(clippy::unwrap_used)]
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .body(Full::from(
            "<h1>Advent of Code API</h1>\n\
              <p>Check the <a href='https://aoc.fornwall.net/api/openapi.json'>OpenAPI document</a>.</p>"
        ))
        .unwrap()
}

async fn handle_post(
    Path(params): Path<HashMap<String, String>>,
    body: String,
) -> Response<Full<Bytes>> {
    #![allow(clippy::unwrap_used)]
    let year = params.get("year").unwrap();
    let day = params.get("day").unwrap();
    let part = params.get("part").unwrap();
    match solve_raw(year, day, part, &body) {
        Ok(solution) => Response::builder()
            .status(StatusCode::OK)
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Type", "text/plain")
            .body(Full::from(solution))
            .unwrap(),
        Err(error) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Access-Control-Allow-Origin", "*")
            .header("content-type", "text/plain")
            .body(Full::from(error))
            .unwrap(),
    }
}
