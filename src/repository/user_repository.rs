use bson::doc;
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use crate::MaaResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaaUser {
    pub user_id: Option<String>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub status: i32,
    pub refresh_jwt_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MaaUserMongo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub status: i32,
    pub refresh_jwt_ids: Vec<String>,
}

impl MaaUser {
    pub fn unknown() -> Self {
        Self {
            user_id: None,
            user_name: "未知用户:(".to_string(),
            email: "unknown@unkown.unkown".to_string(),
            password: "unknown".to_string(),
            status: 0,
            refresh_jwt_ids: vec![],
        }
    }
}

pub struct UserRepository {
    collection: Collection<MaaUser>,
}

impl UserRepository {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("maa_user"),
        }
    }

    pub async fn find_by_email(
        &self,
        email: &str,
    ) -> MaaResult<Option<MaaUser>> {
        let user = self.collection.find_one(doc! {"email": email}).await?;
        Ok(user)
    }

    pub async fn find_by_user_id(
        &self,
        user_id: &str,
    ) -> MaaResult<Option<MaaUser>> {
        let user = self.collection.find_one(doc! {"userId": user_id}).await?;
        Ok(user)
    }

    pub async fn save(&self, user: MaaUser) -> MaaResult<()> {
        self.collection.insert_one(user).await?;
        Ok(())
    }
}
