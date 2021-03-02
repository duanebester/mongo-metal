use futures::{FutureExt, StreamExt};
use mongodb::{error::Error, bson::{bson,doc}, Client, options::ClientOptions};
use serde::{Serialize, Deserialize};
use tokio::sync::{mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use serde_json::Value as JSValue;

extern crate serde_json;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryDatabases {
    pub databases: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCollections {
    pub database: String,
    pub collections: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryProperties {
    pub database: String,
    pub collection: String,
    pub properties: Option<JSValue>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    GetDatabases(QueryDatabases),
    GetCollections(QueryCollections),
    GetProperties(QueryProperties)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventMessage {
    pub kind: String,
    pub event: Event
}

#[derive(Clone, Debug)]
pub struct App {
    pub db: Client
}

impl App {
    pub async fn init(db_url: &str) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(db_url).await?;
        let db = Client::with_options(client_options)?;
    
        // Ping the server to see if we can connect to the cluster
        if let Err(e) = db.database("admin").run_command(doc! {"ping": 1}, None).await {
            error!("{}", e);
            return Err(e);
        }
    
        return Ok(Self {
            db
        });
    }

    pub fn get_client(&self) -> Client {
        return self.db.clone();
    }
}

pub async fn handle_recv_event(db: Client, event_message:EventMessage) -> Result<EventMessage,String> {
    let map_fn = "function(){for(let key in this){emit(key,this[key])}};";
    let reduce_fn = "function(key, values){letresult=[];if(Array.isArray(values)){result=values;}else{result.push(values);}return{name:key,values:result}};";
    match event_message.event {
        Event::GetDatabases(_query) => {
            let db_names = db.list_database_names(None, None).await.unwrap_or_default();
            let query_resp = QueryDatabases {
                databases: Some(db_names)
            };
            let event_resp = EventMessage {
                kind: String::from("GetDatabases"),
                event: Event::GetDatabases(query_resp)
            };
            return Ok(event_resp);
        },
        Event::GetCollections(query) => {
            let db = db.database(&query.database);
            let col_names = db.list_collection_names(None).await.unwrap_or_default();
            let query_resp = QueryCollections {
                database: query.database,
                collections: Some(col_names)
            };
            let event_resp = EventMessage {
                kind: String::from("GetCollections"),
                event: Event::GetCollections(query_resp)
            };
            return Ok(event_resp);
        },
        Event::GetProperties(query) => {
            let database = query.database;
            let collection = query.collection;
            let db = db.database(&database);
            let doc = doc! {"mapReduce": &collection, "map": map_fn, "reduce": reduce_fn, "out": {"inline": 1}};
            let map_reduced = db.run_command(doc, None).await;
            match map_reduced {
                Ok(doc) => {
                    let json: JSValue = bson!(doc).clone().into();
                    let query_resp = QueryProperties {
                        database: database,
                        collection: collection,
                        properties: Some(json)
                    };
                    let event_resp = EventMessage {
                        kind: String::from("GetProperties"),
                        event: Event::GetProperties(query_resp)
                    };
                    return Ok(event_resp);
                },
                Err(_) => {
                    return Err(String::from("Failed to mapReduce"))
                }
            }
        }
    }
}

pub async fn handle_app_connect(db: Client, ws: WebSocket) {
    let (tx, mut app_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (app_tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);
    tokio::task::spawn(rx.forward(tx).map(|result| {
        if let Err(e) = result {
            error!("WebSocket Send Error: {}", e);
        }
    }));

    while let Some(result) = app_rx.next().await {
        match result {
            Ok(msg) => {
                let event: EventMessage = serde_json::from_str(msg.to_str().unwrap()).unwrap();
                debug!("EventMessage: {:?}", event);
                let resp = handle_recv_event(db.clone(), event).await;
                debug!("Response: {:?}", resp);
                match resp {
                    Ok(event) => {
                        let message = serde_json::to_string(&event).unwrap_or(String::from("{\"error\":\"Bad Resp\"}"));
                        app_tx.send(Ok(Message::text(message))).unwrap();
                    },
                    Err(err) => error!("WebSocket Send Error: {}", err)
                }
                
                // echo message back
                // if let Err(_disconnected) = app_tx.send(Ok(msg.clone())) {
                //     debug!("App Disconnected")
                // }
            }
            Err(e) => {
                error!("WebSocket Error: {}",  e);
                break;
            }
        };
    }
}


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let app:App = App::init("mongodb://localhost:27017/map").await.unwrap();

    let routes = warp::path("ws")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .and(with_client(app))
        .map(|ws: warp::ws::Ws, db: Client| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(move |websocket| handle_app_connect(db, websocket))
        });

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

fn with_client(app: App) -> impl Filter<Extract = (Client,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || app.get_client())
}
