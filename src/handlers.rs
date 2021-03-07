use crate::{db::DB, WebResult};
// use super::{WebResult};
use warp::{reject, reply::json, Reply};

pub async fn list_databases(db: DB) -> WebResult<impl Reply> {
    let databases = db.fetch_database_names().await.map_err(|e| reject::custom(e))?;
    Ok(json(&databases))
}

pub async fn list_collections(database: String, db: DB) -> WebResult<impl Reply> {
    let collections = db.fetch_collection_names(&database).await.map_err(|e| reject::custom(e))?;
    Ok(json(&collections))
}

pub async fn list_properties(database: String, collection: String, db: DB) -> WebResult<impl Reply> {
    let properties = db.fetch_collection_props(&database, &collection).await.map_err(|e| reject::custom(e))?;
    Ok(json(&properties))
}

pub async fn list_all_collections_properties(database: String, db: DB) -> WebResult<impl Reply> {
    let properties = db.fetch_all_collection_props(&database).await.map_err(|e| reject::custom(e))?;
    Ok(json(&properties))
}