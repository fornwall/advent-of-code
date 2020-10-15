use advent_of_code::solve_raw;
use std::convert::Infallible;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = 8080;

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST"]);

    let post_input_route = warp::path!(String / String / String)
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
    Ok(if let Ok(input) = std::str::from_utf8(body.as_ref()) {
        match solve_raw(&year, &day, &part, input) {
            Ok(solution) => warp::http::response::Response::builder()
                .header("content-type", "text/plain")
                .status(warp::http::StatusCode::OK)
                .body(solution),
            Err(error) => warp::http::response::Response::builder()
                .header("content-type", "text/plain")
                .status(warp::http::StatusCode::BAD_REQUEST)
                .body(error),
        }
    } else {
        warp::http::response::Response::builder()
            .status(warp::http::StatusCode::BAD_REQUEST)
            .body("Invalid utf-8".to_string())
    })
}
