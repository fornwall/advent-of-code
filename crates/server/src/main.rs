use advent_of_code::solve_raw;
use warp::Filter;

#[tokio::main]
async fn main() {
    let port = 8080;
    println!("Running on port {}", port);
    // GET /hello/warp => 200 OK with body "Hello, warp!"

    let hello = warp::path!(String / String / String)
        .and(warp::post())
        .and(warp::body::bytes())
        .map(
            |year: String, day: String, part: String, body: warp::hyper::body::Bytes| {
                if let Ok(input) = std::str::from_utf8(body.as_ref()) {
                    match solve_raw(&year, &day, &part, input) {
                        Ok(solution) => warp::http::response::Response::builder()
                            .header("content-type", "text")
                            .status(warp::http::StatusCode::OK)
                            .body(solution),
                        Err(error) => warp::http::response::Response::builder()
                            .header("content-type", "text")
                            .status(warp::http::StatusCode::BAD_REQUEST)
                            .body(error),
                    }
                } else {
                    warp::http::response::Response::builder()
                        .status(warp::http::StatusCode::BAD_REQUEST)
                        .body("Invalid utf-8".to_string())
                }
            },
        );

    warp::serve(hello).run(([0, 0, 0, 0], port)).await;
}
