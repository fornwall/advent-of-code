use advent_of_code::solve_raw;
use std::convert::Infallible;
use warp::Filter;

#[tokio::main]
async fn main() {
    #![allow(clippy::expect_used)]

    let port = 8080;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);

    let post_input_route = warp::path!("solve" / String / String / String)
        .and(warp::body::bytes())
        .and_then(handle_post);

    let get_all_route = warp::any().and_then(handle_get);

    let routes = post_input_route.or(get_all_route).with(cors);

    println!("Running on port {}", port);
    warp::serve(routes).run(([0, 0, 0, 0], port)).await;
}

async fn handle_get() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::http::response::Response::builder()
        .header("content-type", "text/html")
        .status(warp::http::StatusCode::OK)
        .body("<h1>Advent of Code API</h1>\n\
              <p>Check the <a href='https://aoc.fornwall.net/openapi.json'>OpenAPI document</a>.</p>"))
}

async fn handle_post(
    year: String,
    day: String,
    part: String,
    body: warp::hyper::body::Bytes,
) -> Result<impl warp::Reply, Infallible> {
    let response = warp::http::response::Response::builder()
        .header("content-type", "text/plain; charset=utf-8 ")
        .header("cache-control", "no-cache");
    Ok(if let Ok(input) = std::str::from_utf8(body.as_ref()) {
        match solve_raw(&year, &day, &part, input) {
            Ok(solution) => response
                .status(warp::http::StatusCode::OK)
                .header("content-length", solution.len())
                .body(solution),
            Err(error) => response
                .header("content-length", error.len())
                .status(warp::http::StatusCode::BAD_REQUEST)
                .body(error),
        }
    } else {
        let message = "Invalid utf-8".to_string();
        response
            .status(warp::http::StatusCode::BAD_REQUEST)
            .header("content-length", message.len())
            .body("Invalid utf-8".to_string())
    })
}
