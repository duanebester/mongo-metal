use warp::{Filter};
use crate::db::DB;
use std::convert::Infallible;

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}