use bson::Document;
use serde::Serialize;

use crate::MaaResult;

pub mod ark_level_repository;
pub mod github_api;
pub mod redis_connection_manager;

pub trait MongoDocument: Serialize {
    fn get_document(&self) -> MaaResult<Document> {
        bson::to_document(self).map_err(Into::into)
    }
}
