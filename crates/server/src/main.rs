use advent_of_code::solve_raw;
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!(String / String / String)
        .and(warp::post())
        .and(warp::body::bytes())
        .map(
            |year: String, day: String, part: String, body: warp::hyper::body::Bytes| {
                if let Ok(input) = std::str::from_utf8(body.as_ref()) {
                    match solve_raw(&year, &day, &part, input) {
                        Ok(solution) => solution,
                        Err(error) => error,
                    }
                } else {
                    "Invalid utf-8".to_string()
                }
            },
        );

    warp::serve(hello).run(([0, 0, 0, 0], 8080)).await;
}
