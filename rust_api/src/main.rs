use warp::{http::Method, Filter, Rejection};

mod manage_pool;
mod response;
mod handler;
use manage_pool::{DB};

// the result would either be any type T or Rejection
type WebResult<T> = std::result::Result<T, Rejection>;
//idk
#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "api=nfo");
    }
    pretty_env_logger::init();

    let db = manage_pool::pool_db();

    let hello = warp::path!("api"/ "hello").and(warp::get()).and_then(handler::hello);
    
    let pools_router = warp::path!("api" / "pools");

    let cors = warp::cors().allow_methods(&[Method::GET, Method::POST])
    .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
    .allow_headers(vec!["content-type"]);
    
    let routes = hello.with(warp::log("api"));
    let pool_routes = pools_router
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db.clone()))
    .and_then(handler::receive_pool_handler)
    .or(pools_router
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(handler::pool_request_list));
   
    let routes = pool_routes
        .with(cors)
        .with(warp::log("api"))
        .or(hello);

    println!("Server started");
    warp::serve(routes).run(([0,0,0,0], 8000)).await;
}

fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}