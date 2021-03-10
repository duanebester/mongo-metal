use crate::{db::DB, WebResult};
use std::collections::HashMap;
use mongodb::{bson::{doc, bson, from_bson}};
use serde::{Deserialize, Serialize};
use serde_json::{Value as JsValue};
use warp::{reject, reply::json, Reply};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MapReducedValue {
    pub name: String,
    pub values: Vec<JsValue>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MapReducedCollection {
    pub name: String,
    pub values: Vec<JsValue>,
    pub collection: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MapReducedResults {
    pub value: MapReducedCollection
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MapReduced {
    pub results: Vec<MapReducedResults>,
}

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
    let map_reduced: HashMap<String,Vec<MapReducedValue>> = properties
        .iter()
        .map(|prop| {
            let mr: MapReduced = from_bson(bson!(prop)).unwrap();
            mr.results
        })
        .flatten()
        .filter(|res| res.value.name != "__v")
        .map(|res| {
            let val = MapReducedValue {
                name: res.value.name,
                values: res.value.values
            };
            (res.value.collection, vec![val])
        }).fold(HashMap::new(), |mut hmap, pair| {
            let (name, vals) = pair;
            hmap.entry(name.to_string()).or_insert(Vec::new()).extend(vals.to_vec());
            hmap
        });

    Ok(json(&map_reduced))
}