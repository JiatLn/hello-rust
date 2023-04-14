use std::env;
use warp::Filter;
extern crate pretty_env_logger;

#[macro_use]
extern crate log;

#[tokio::main]
async fn main() {
    // log
    env::set_var("RUST_APP_LOG", "debug");
    pretty_env_logger::init_custom_env("RUST_APP_LOG");
    let log = warp::log("apis");

    // for test
    warn!("oh! no");

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));

    // GET / => 200 OK with body "Hello, world!"
    let index = warp::get().and(warp::path::end()).and_then(index_handler);

    let routes = index.or(hello).with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn index_handler() -> Result<impl warp::Reply, warp::Rejection> {
    Ok("Hello, world!")
}
