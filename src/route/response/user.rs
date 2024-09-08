use serde::Serialize;

use crate::repository::user_repository::MaaUser;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaaUserInfo {
    pub id: String,
    pub user_name: String,
    pub activated: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaaLoginResponse {
    pub token: String,
    pub valid_before: i64,
    pub valid_after: i64,
    pub refresh_token: String,
    pub refresh_token_valid_before: i64,
    pub refresh_token_valid_after: i64,
    pub user_info: MaaUserInfo,
}

impl From<MaaUser> for MaaUserInfo {
    fn from(user: MaaUser) -> Self {
        Self {
            id: user.user_id.unwrap_or_default(),
            user_name: user.user_name,
            activated: user.status == 1,
        }
    }
}
