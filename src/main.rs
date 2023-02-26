use warp::{Filter, path::FullPath, reply, http::StatusCode};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let index = warp::fs::file("./build/index.html");

    // if doesnt have extensions return index.
    let short_circuit_extensions = warp::path::full()
        .and_then(move |path: FullPath| async move {
        if path.as_str().to_string().contains(".json") {
            Ok(reply::with_status("", StatusCode::NOT_FOUND))
        } else {
            Err(warp::reject::not_found())
        }
    });

    let routes = warp::fs::dir("./build")
    .with(warp::reply::with::header("Cache-Control", "max-age=31536000"))
    .with(warp::filters::compression::gzip())
    .or(short_circuit_extensions).or(index);

    warp::serve(routes).run(([0, 0, 0, 0], 3000)).await;
}
