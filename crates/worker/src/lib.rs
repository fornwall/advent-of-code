#![allow(clippy::future_not_send)]
use advent_of_code::solve_raw;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf()
            .map_or((0., 0.), |cf| cf.coordinates().unwrap_or_default()),
        req.cf().map_or("?".into(), |cf| cf
            .region()
            .unwrap_or_else(|| "unknown region".into()))
    );
}

#[event(fetch)]
#[allow(clippy::unwrap_used)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .post_async("/solve/:year/:day/:part", |mut req, ctx| async move {
            let year = ctx.param("year").unwrap();
            let day = ctx.param("day").unwrap();
            let part = ctx.param("part").unwrap();
            let input = req.text().await?;

            let response = match solve_raw(year, day, part, &input) {
                Ok(answer) => Response::ok(answer),
                Err(message) => Response::error(message, 400),
            }
            .unwrap();

            let headers = Headers::new();
            headers.append("Access-Control-Allow-Origin", "*").unwrap();
            Ok(response.with_headers(headers))
        })
        .get("/worker-version", |_, _ctx| {
            let version = env!("CARGO_PKG_VERSION");
            Response::ok(version)
        })
        .run(req, env)
        .await
}
