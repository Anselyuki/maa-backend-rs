use bson::Document;
use serde::Serialize;

use crate::MaaResult;

pub mod ark_level_repository;
pub mod github_api;
pub mod mongo_repository;

pub trait MongoDocument: Serialize {
    fn get_document(&self) -> MaaResult<Document> {
        bson::to_document(self).map_err(Into::into)
    }
}
