mod model;
mod store;

use std::collections::HashMap;

use model::Question;
use store::Store;
use warp::{cors::CorsForbidden, http::Method, hyper::StatusCode, Filter, Rejection, Reply};

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl Reply, Rejection> {
    let mut start = 0;
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Fuck");
    }
    println!("{}", start);
    let v: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&v))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([&Method::POST, &Method::DELETE, &Method::GET, &Method::PUT]);

    let hello = warp::get()
        .and(warp::path("questions"))
        .and(warp::query())
        .and(store_filter)
        .and_then(get_questions)
        .recover(return_error);

    let routes = hello.with(cors);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
