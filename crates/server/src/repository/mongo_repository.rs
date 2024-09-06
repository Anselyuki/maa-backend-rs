use mongodb::{Client, Database};

use crate::{envs::db_uri, MaaError, MaaResult};

pub struct MongoRepository {
    client: Client,
    db: Database,
}

impl MongoRepository {
    pub async fn new() -> MaaResult<Self> {
        let uri = db_uri()?;
        let client = Client::with_uri_str(&uri).await?;
        let db = client
            .default_database()
            .ok_or(MaaError::NoDefaultDBError)?;

        Ok(Self { client, db })
    }
}
