use std::collections::HashMap;
use crate::{errors::Error::MongoQueryError, Result};
use mongodb::{options::ClientOptions, Client, bson::{doc, Document}};
use futures::{FutureExt, TryFutureExt};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
    map_fn: String,
    reduce_fn: String
}

impl DB {
    pub async fn init(db_url: &str) -> Result<Self> {
        let client_options = ClientOptions::parse(db_url).await?;
        let client = Client::with_options(client_options)?;
        client.database("admin").run_command(doc! {"ping": 1}, None).await?;

        log::debug!("Connected to Database");

        Ok(Self {
            client: client,
            map_fn: String::from("function(){for(let key in this){emit(key,this[key])}};"),
            reduce_fn: String::from("function(key, values){letresult=[];if(Array.isArray(values)){result=values;}else{result.push(values);}return{name:key,values:result}};"),
        })
    }

    pub async fn fetch_database_names(&self) -> Result<Vec<String>> {
        let db_names = self.client.list_database_names(None, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(db_names)
    }

    pub async fn fetch_collection_names(&self, database: &str) -> Result<Vec<String>> {
        let db = self.client.database(database);
        let col_names = db.list_collection_names(None)
            .await
            .map_err(MongoQueryError)?;
        Ok(col_names)
    }

    pub async fn fetch_collection_props(&self, database: &str, collection: &str) -> Result<Document> {
        let db = self.client.database(database);
        let map_reduce = doc! {"mapReduce": collection, "map": &self.map_fn, "reduce": &self.reduce_fn, "out": {"inline": 1}};
        let map_reduced = db.run_command(map_reduce, None)
            .await
            .map_err(MongoQueryError)?;
        Ok(map_reduced)
    }

    pub async fn fetch_all_collection_props(&self, database: &str) -> Result<HashMap<String,Document>> {
        let db = self.client.database(database);
        let get_names_props = db.list_collection_names(None)
            .map_err(MongoQueryError)
            .map_ok(|names| {
                names.into_iter().map(|collection| {
                    let map_reduce = doc! {"mapReduce": &collection, "map": &self.map_fn, "reduce": &self.reduce_fn, "out": {"inline": 1} };
                    let map_reduced = db.run_command(map_reduce, None)
                        .map_err(MongoQueryError)
                        .map_ok(|res: Document| -> Result<(String, Document)> {
                            Ok((collection, res)) // Tuple - Collection Name, Collection Properties
                        });
                    map_reduced
                }).collect::<Vec<_>>()
            });

        let get_props = get_names_props.await?;
        let results = futures::future::try_join_all(get_props)
            .await?
            .into_iter()
            .map(Result::unwrap) // Ignore errors?
            .collect::<HashMap<String,Document>>();

        Ok(results)
    }
}
