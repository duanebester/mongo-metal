use warp::{Filter, Rejection};
use std::env;
use db::DB;
use filters::with_db;

type Result<T> = std::result::Result<T, errors::Error>;
type WebResult<T> = std::result::Result<T, Rejection>;

mod db;
mod errors;
mod filters;
mod handlers;

extern crate pretty_env_logger;
extern crate log;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=mongo=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "mongo=info");
    }
    pretty_env_logger::init();

    let db_url = option_env!("MONGODB_URI").unwrap_or("mongodb://localhost:27017");
    let db = DB::init(db_url).await?;

    // /databases
    let databases = warp::path("databases")
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(handlers::list_databases);

    // /databases/:database/collections
    let collections = warp::path!("databases" / String / "collections")
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(handlers::list_collections);

    // /databases/:database/properties
    let all_properties = warp::path!("databases" / String / "properties")
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(handlers::list_all_collections_properties);

    // /databases/:database/collections/:collection/properties
    let properties = warp::path!("databases" / String / "collections" / String / "properties")
        .and(warp::path::end())
        .and(with_db(db.clone()))
        .and_then(handlers::list_properties);

    let routes = warp::get()
        .and(warp::path("api")
            .and(databases.or(collections).or(all_properties).or(properties)))
            .with(warp::cors().allow_any_origin())
            .with(warp::log("mongo"))
            .recover(errors::handle_rejection);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
    Ok(())
}