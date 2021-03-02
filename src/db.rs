use mongodb::{options::ClientOptions, Client, error::Error, bson::doc};

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    pub async fn init(db_url: &str) -> Result<Self, Error> {
        let client_options = ClientOptions::parse(db_url).await?;
        let client = Client::with_options(client_options)?;
    
        // Ping the server to see if we can connect to the cluster
        if let Err(e) = client.database("admin").run_command(doc! {"ping": 1}, None).await {
            error!("{}", e);
            return Err(e);
        }
    
        return Ok(Self {
            client
        });
    }

    pub fn get_client(&self) -> Client {
        return self.client.clone();
    }
}